//! Contains the implementation of the decryption side of `envcrypt!`

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};

/// Decrypts some bytes from a given key and nonce using exactly the same
/// logic as [`envc!`] and [`option_envc!`].
///
/// DO NOT CALL THIS FUNCTION YOURSELF. Decryption is handled automatically
/// by the [`envc!`] and [`option_envc!`] macros at runtime.
pub fn decrypt(key: &[u8], encrypted_variable: &[u8], nonce: &[u8]) -> &'static str {
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();

    let decrypted_buffer = cipher
        .decrypt(Nonce::from_slice(nonce), encrypted_variable)
        .unwrap();

    let decrypted_variable = String::from_utf8(decrypted_buffer)
        .unwrap()
        .into_boxed_str();

    Box::leak(decrypted_variable)
}
