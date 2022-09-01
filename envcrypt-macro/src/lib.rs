use std::env;

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use rand::{
    distributions::{DistString, Standard},
    rngs::OsRng,
};

#[proc_macro_error]
#[proc_macro]
pub fn envcrypt(tokens: TokenStream) -> TokenStream {
    let variable = get_variable(tokens);

    let EncryptedVariable { key, iv, encrypted } = encrypt(variable);

    quote! {
        {
            envcrypt::__internal::decrypt(#key, #iv, #encrypted)
        }
    }
    .into()
}

struct EncryptedVariable {
    key: Literal,
    iv: Literal,
    encrypted: Literal,
}

fn encrypt(variable: String) -> EncryptedVariable {
    let key = Standard.sample_string(&mut OsRng, 256);
    let iv = Standard.sample_string(&mut OsRng, 256);

    let magic = MagicCrypt256::new(&key, Some(&iv));
    let encrypted = magic.encrypt_str_to_bytes(variable);

    EncryptedVariable {
        key: Literal::string(&key),
        iv: Literal::string(&iv),
        encrypted: Literal::byte_string(&encrypted),
    }
}

macro_rules! syntax_error {
    () => {
        abort_call_site!("Invalid syntax. Valid forms are `envcrypt!(\"KEY\")` and `envcrypt!(\"KEY\", \"error message\")`")
    };
}

fn get_variable(tokens: TokenStream) -> String {
    let mut iter = tokens.into_iter();

    let key = if let Some(TokenTree::Literal(literal)) = iter.next() {
        let with_quotes = literal.to_string();
        with_quotes[1..with_quotes.len() - 1].to_owned()
    } else {
        syntax_error!()
    };

    let abort_call_site_message = match (iter.next(), iter.next(), iter.next()) {
        (Some(TokenTree::Punct(comma)), Some(TokenTree::Literal(message)), None)
            if comma.as_char() == ',' =>
        {
            let with_quotes = message.to_string();
            with_quotes[1..with_quotes.len() - 1].to_owned()
        }
        (None, None, None) => format!("environment variable `{}` not defined", &key),
        _ => syntax_error!(),
    };

    if let Ok(var) = env::var(&key) {
        var
    } else {
        abort_call_site!("{}", abort_call_site_message)
    }
}
