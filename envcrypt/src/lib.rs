#![deny(missing_docs, clippy::missing_docs_in_private_items)]
#![cfg_attr(windows, doc = include_str!("..\\..\\README.md"))]
#![cfg_attr(not(windows), doc = include_str!("../../README.md"))]

/// Internal, not for public consumption
#[doc(hidden)]
pub mod __internal {
    use std::str::from_utf8;

    use magic_crypt::{MagicCrypt256, MagicCryptTrait};

    /// Decrypt
    pub fn decrypt(key: &[u8], iv: &[u8], encrypted_value: &[u8]) -> String {
        let key_str = from_utf8(key).unwrap();
        let iv_str = from_utf8(iv).unwrap();

        let magic = MagicCrypt256::new(key_str, Some(iv_str));
        let decrypted = magic.decrypt_bytes_to_bytes(encrypted_value).unwrap();

        String::from_utf8(decrypted).unwrap()
    }
}

/// Inspects and encrypts an environment variable at compile time
/// and decrypts at runtime.
///
/// This macro will expand to the value of the named environment variable at compile time,
/// yielding an expression of type `String`.
/// Use [`std::env::var`] instead if you want to read the value at runtime.
///
/// ```rust
/// # use envcrypt::envc;
/// let path: String = envc!("PATH");
/// println!("the $PATH variable at the time of compiling was: {path}");
/// ```
///
///  If the environment variable is not defined, then a compilation error will be emitted.
/// To not emit a compile error, use the [`option_envc!`] macro instead.
///
/// ```compile_fail
/// # use envcrypt::envc;
/// let unlikely_variable: String = envc!("HIGHLY_UNLIKELY_ENVIRONMENT_VARIABLE");
/// ```
#[doc(inline)]
pub use envcrypt_macro::envc;

/// Optionally inspects and encrypts an environment variable at compile time
/// and decrypts at runtime.
///
/// If the named environment variable is present at compile time,
/// this will expand into an expression of type `Option<String>`
/// whose value is `Some` of the value of the environment variable.
/// If the environment variable is not present, then this will expand to `None`.
///
/// Use [`std::env::var`] instead if you want to read the value at runtime.
///
/// A compile time error is never emitted when using this macro
/// regardless of whether the environment variable is present or not.
///
/// ```rust
/// # use envcrypt::option_envc;
/// let key: Option<String> = option_envc!("SECRET_KEY");
/// println!("the secret key might be: {key:?}");
/// ```
#[doc(inline)]
pub use envcrypt_macro::option_envc;
