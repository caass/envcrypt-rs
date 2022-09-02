//! Contains the implementation of the encryption side of `envcrypt!`

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use proc_macro2::Literal;

/// Represents an encrypted environment variable
pub struct EncryptedVariable {
    /// The encryption key that was used to encrypt the variable
    pub key: Literal,

    /// The encrypted value of the variable
    pub value: Literal,

    /// The nonce that was used to encrypt the variable
    pub nonce: Literal,
}

/// Encrypts an environment variable.
///
/// The implementation here should be the exact inverse of `envcrypt::__internal::decrypt`
pub fn encrypt(unencrypted_variable: String) -> EncryptedVariable {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let encrypted_variable = cipher
        .encrypt(&nonce, unencrypted_variable.as_bytes())
        .unwrap();

    EncryptedVariable {
        key: Literal::byte_string(key.as_ref()),
        value: Literal::byte_string(encrypted_variable.as_ref()),
        nonce: Literal::byte_string(nonce.as_ref()),
    }
}
