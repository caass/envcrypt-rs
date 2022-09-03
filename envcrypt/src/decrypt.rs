//! Contains the implementation of the decryption side of `envcrypt!`

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};

/// Decrypts some bytes from a given key and nonce using exactly the same
/// logic as [`envc!`] and [`option_envc!`].
///
/// DO NOT CALL THIS FUNCTION YOURSELF. Decryption is handled automatically
/// by the [`envc!`] and [`option_envc!`] macros at runtime.
pub fn decrypt(data: &[u8]) -> &'static str {
    let key = Key::from_slice(&data[..32]);
    let nonce = Nonce::from_slice(&data[32..44]);
    let ciphertext = &data[44..];

    let cipher = ChaCha20Poly1305::new(key);

    let decrypted_buffer = cipher.decrypt(nonce, ciphertext).unwrap();

    let decrypted_variable = String::from_utf8(decrypted_buffer)
        .unwrap()
        .into_boxed_str();

    Box::leak(decrypted_variable)
}
