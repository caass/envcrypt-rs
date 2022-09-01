#![cfg_attr(not(doctest), doc = include_str!("../../README.md"))]

#[doc(hidden)]
pub mod __internal {
    use magic_crypt::{MagicCrypt256, MagicCryptTrait};

    #[doc(hidden)]
    pub fn decrypt(
        key: impl AsRef<str>,
        iv: impl AsRef<str>,
        encrypted_value: impl AsRef<str>,
    ) -> String {
        let magic = MagicCrypt256::new(key.as_ref(), Some(iv.as_ref()));
        magic
            .decrypt_base64_to_string(encrypted_value.as_ref())
            .unwrap()
    }
}

/// Encrypt an environment variable at compile time,
/// and decrypt it at runtime.
pub use envcrypt_macro::envcrypt;
