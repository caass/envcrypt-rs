#![deny(missing_docs, clippy::missing_docs_in_private_items)]

//! Do not use this crate directly, it will not work.
//! Use [`envcrypt`](https://crates.io/crate/envcrypt) instead.

use std::env::{self, VarError};

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use rand::{
    distributions::{DistString, Standard},
    rngs::OsRng,
};

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

#[allow(missing_docs)] // documented in main crate
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
        Ok(variable) => {
            let EncryptedVariable { key, iv, encrypted } = encrypt(variable);
            quote!({ envcrypt::__internal::decrypt(#key, #iv, #encrypted) })
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

#[allow(missing_docs)] // documented in main crate
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
        Ok(variable) => {
            let EncryptedVariable { key, iv, encrypted } = encrypt(variable);
            quote!(::std::option::Option::Some({ ::envcrypt::__internal::decrypt(#key, #iv, #encrypted) }))
        }

        Err(VarError::NotUnicode(_)) => {
            abort_call_site!(
                "Environment variable ${} contains non-unicode value",
                &env_var_key
            )
        }

        Err(VarError::NotPresent) => quote!(::std::option::Option::<String>::None),
    }
    .into()
}

/// Returns `Some(value)` if the provided literal was a string literal,
/// or `None` otherwise.
fn stringify(literal: &proc_macro::Literal) -> Option<String> {
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

/// Represents an encrypted environment variable
struct EncryptedVariable {
    /// The encryption key that was used to encrypt the variable
    key: Literal,

    /// The initialization vector that was used to encrypt the variable
    iv: Literal,

    /// The encrypted value of the variable
    encrypted: Literal,
}

/// Encrypts an environment variable
fn encrypt(variable: String) -> EncryptedVariable {
    let key = Standard.sample_string(&mut OsRng, 256);
    let iv = Standard.sample_string(&mut OsRng, 256);

    let magic = MagicCrypt256::new(&key, Some(&iv));
    let encrypted = magic.encrypt_str_to_bytes(variable);

    EncryptedVariable {
        key: Literal::byte_string(key.as_bytes()),
        iv: Literal::byte_string(iv.as_bytes()),
        encrypted: Literal::byte_string(&encrypted),
    }
}
