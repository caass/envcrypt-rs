use std::env;

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use quote::quote;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

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
    let key = OsRng
        .sample_iter(Alphanumeric)
        .take(256)
        .map(char::from)
        .collect::<String>();
    let iv = OsRng
        .sample_iter(Alphanumeric)
        .take(256)
        .map(char::from)
        .collect::<String>();

    let magic = MagicCrypt256::new(&key, Some(&iv));
    let encrypted = magic.encrypt_str_to_base64(variable);

    EncryptedVariable {
        key: Literal::string(&key),
        iv: Literal::string(&iv),
        encrypted: Literal::string(&encrypted),
    }
}

fn get_variable(tokens: TokenStream) -> String {
    let key = if let Some(TokenTree::Literal(literal)) = tokens.into_iter().next() {
        let with_quotes = literal.to_string();
        with_quotes[1..with_quotes.len() - 1].to_owned()
    } else {
        panic!("Expected a string literal")
    };

    if let Ok(var) = env::var(&key) {
        var
    } else {
        panic!("Failed to find environment variable {key}")
    }
}
