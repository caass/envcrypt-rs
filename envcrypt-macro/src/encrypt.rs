//! Contains the implementation of the encryption side of `envcrypt!`

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;

/// Encrypts an environment variable and returns a tokenized byte array of the encrypted
/// variable along with a key and nonce to decrypt it.
///
/// The implementation here should be the exact inverse of `envcrypt::__internal::decrypt`
pub(crate) fn encrypt<V: Into<UnencryptedVariable>>(v: V) -> TokenStream {
    let unencrypted_variable: UnencryptedVariable = v.into();
    if let Some(variable) = unencrypted_variable.inner() {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let cipher = ChaCha20Poly1305::new(&key);

        let encrypted_variable = cipher.encrypt(&nonce, variable.as_bytes()).unwrap();

        let data = key
            .into_iter()
            .chain(nonce.into_iter())
            .chain(encrypted_variable.into_iter())
            .collect::<Vec<_>>();

        let bytestring = Literal::byte_string(&data);

        if unencrypted_variable.is_optional() {
            quote!(::core::option::Option::Some(::envcrypt::__internal::decrypt(#bytestring)))
        } else {
            quote!(::envcrypt::__internal::decrypt(#bytestring))
        }
    } else {
        quote!(::core::option::Option::<&'static str>::None)
    }
    .into()
}

/// The types of environment variables we can encrypt: optional ones (via `option_envc!`) and required
/// ones (via `envc!`)
pub(crate) enum UnencryptedVariable {
    /// A required environment variable
    Required(String),

    /// An optional environment variable
    Optional(Option<String>),
}

impl From<String> for UnencryptedVariable {
    fn from(unencrypted_variable: String) -> Self {
        UnencryptedVariable::Required(unencrypted_variable)
    }
}

impl From<Option<String>> for UnencryptedVariable {
    fn from(option_unencrypted_variable: Option<String>) -> Self {
        UnencryptedVariable::Optional(option_unencrypted_variable)
    }
}

impl UnencryptedVariable {
    /// Get the inner variable, if it exists
    fn inner(&self) -> Option<&String> {
        match self {
            UnencryptedVariable::Optional(ref option) => option.as_ref(),
            UnencryptedVariable::Required(ref v) => Some(v),
        }
    }

    /// Check if the variable is optional
    fn is_optional(&self) -> bool {
        matches!(self, UnencryptedVariable::Optional(_))
    }
}
