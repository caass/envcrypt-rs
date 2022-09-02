#![deny(missing_docs, clippy::missing_docs_in_private_items)]
#![cfg_attr(windows, doc = include_str!("..\\README.md"))]
#![cfg_attr(not(windows), doc = include_str!("../README.md"))]

use std::env::{self, VarError};

use proc_macro::{Literal, TokenStream, TokenTree};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;

mod decrypt;
mod encrypt;
use decrypt::{decrypt, decrypt_none, decrypt_some};
use encrypt::{encrypt, EncryptedVariable};

/// Shortcut for aborting due to a syntax error.
macro_rules! syntax_error {
    ($macro_name:ident) => {
        match $macro_name {
            "envc" => abort_call_site!("Invalid syntax. Valid forms are `envc!(\"VAR_NAME\")` and `envc!(\"VAR_NAME\", \"custom error message\")`."),

            "option_envc" => abort_call_site!("Invalid syntax. Expected input of the form `option_envc!(\"VAR_NAME\")`"),

            _ => panic!("Unknown macro name")
        }
    };
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro]
pub fn envc(tokens: TokenStream) -> TokenStream {
    let (env_var_key, abort_message) = match parse(tokens, "envc") {
        Input::VariableName(variable_name) => (
            variable_name.clone(),
            format!("environment variable `{}` not defined", &variable_name),
        ),
        Input::VariableNameAndAbortMessage {
            variable_name,
            abort_message,
        } => (variable_name, abort_message),
    };

    match env::var(&env_var_key) {
        Ok(unencrypted_variable) => {
            let EncryptedVariable { key, value, nonce } = encrypt(unencrypted_variable);
            decrypt!(key, value, nonce)
        }

        Err(VarError::NotUnicode(_)) => {
            abort_call_site!(
                "Environment variable ${} contains non-unicode value",
                &env_var_key
            )
        }

        Err(VarError::NotPresent) => abort_call_site!("{}", abort_message),
    }
    .into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro]
pub fn option_envc(tokens: TokenStream) -> TokenStream {
    let env_var_key = match parse(tokens, "option_envc") {
        Input::VariableName(variable_name) => variable_name,
        Input::VariableNameAndAbortMessage { .. } => abort_call_site!(
            "Invalid syntax. Expected input of the form `option_envc!(\"VAR_NAME\")`"
        ),
    };

    match env::var(&env_var_key) {
        Ok(unencrypted_variable) => {
            let EncryptedVariable { key, value, nonce } = encrypt(unencrypted_variable);
            decrypt_some!(key, value, nonce)
        }

        Err(VarError::NotUnicode(_)) => {
            abort_call_site!(
                "Environment variable ${} contains non-unicode value",
                &env_var_key
            )
        }

        Err(VarError::NotPresent) => decrypt_none!(),
    }
    .into()
}

/// Returns `Some(value)` if the provided literal was a string literal,
/// or `None` otherwise.
fn stringify(literal: &Literal) -> Option<String> {
    let stringified = literal.to_string();
    if stringified.starts_with('"') && stringified.ends_with('"') {
        Some(stringified[1..stringified.len() - 1].to_owned())
    } else {
        None
    }
}

/// Possible inputs to the [`envc!`] and [`option_envc!`] macros.
enum Input {
    /// A variable to inspect at compile time
    VariableName(String),

    /// A variable to inspect at compile time and a custom abort message if it's missing
    VariableNameAndAbortMessage {
        /// The variable to inspect
        variable_name: String,

        /// The message to display if the variable is missing
        abort_message: String,
    },
}

/// Parses a [`TokenStream`] into [`Input`]
fn parse(tokens: TokenStream, macro_name: &str) -> Input {
    let tokens_vec = tokens.into_iter().collect::<Vec<_>>();

    match *tokens_vec.as_slice() {
        // `envc!("MY_VAR")`
        [TokenTree::Literal(ref variable_literal)] => {
            if let Some(variable) = stringify(variable_literal) {
                Input::VariableName(variable)
            } else {
                syntax_error!(macro_name)
            }
        }

        // `envc!("MY_VAR", "custom error message")
        [TokenTree::Literal(ref variable_literal), TokenTree::Punct(ref comma), TokenTree::Literal(ref message_literal)] => {
            match (
                stringify(variable_literal),
                comma.as_char(),
                stringify(message_literal),
            ) {
                (Some(variable_name), ',', Some(abort_message)) => {
                    Input::VariableNameAndAbortMessage {
                        variable_name,
                        abort_message,
                    }
                }
                _ => syntax_error!(macro_name),
            }
        }
        _ => syntax_error!(macro_name),
    }
}
