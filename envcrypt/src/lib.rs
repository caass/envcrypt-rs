#![deny(missing_docs, clippy::missing_docs_in_private_items)]

//! Drop-in replacements for [`env!`] and [`option_env!`]
//! that encrypt your variables at compile-time and decrypt them at runtime.
//!
//! While it's still possible to reverse-engineer the values, `envcrypt` prevents
//! `strings <my-binary>` from trivially finding embedded secrets.
//!
//! Since the secret must be decrypted at runtime,
//! `envc!` returns an owned [`String`] instead of `&'static str`.
//! Its API otherwise mirrors [`env!`] and [`option_env!`].
//!
//! # Usage
//!
//! As a replacement for [`env!`]
//!
//! ```rust
//! use envcrypt::envc;
//!
//! let my_super_secret_key: String = envc!("SECRET_KEY");
//! // ...do stuff with your secret key
//! ```
//!
//! As a replacement for [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html)
//!
//! ```rust
//! use envcrypt::option_envc;
//!
//! if let Some(optional_value) = option_envc!("OPTIONAL_SECRET_KEY") {
//!   // ...do stuff
//! }
//! ```
//!
//! With [`dotenvy`](https://crates.io/crates/dotenvy):
//!
//! `.env`:
//!
//! ```dotenv
//! CLIENT_SECRET="my_client_secret"
//! SOME_TOKEN="some_token"
//! ```
//!
//! `build.rs`:
//!
//! ```
//! # #[allow(clippy::needless_doctest_main)]
//! use dotenvy::dotenv_iter;
//!
//! fn main(){
//!  println!("cargo:rerun-if-changed=.env");
//!
//!  for item in dotenv_iter().unwrap() {
//!    let (key, value) = item.unwrap();
//!    println!("cargo:rustc-env=${key}=${value}");
//!  }
//!
//! }
//! ```
//!
//! `main.rs`:
//!
//! ```rust
//! use envcrypt::envc;
//!
//! let client_secret: String = envc!("CLIENT_SECRET");
//! ```
//!
//! # Details
//!
//! Encryption is powered by [`magic_crypt`] using AES-256 encryption.
//!
//! Inspired by [`litcrypt`](https://docs.rs/crate/litcrypt/0.3.0)

#[doc(hidden)]
pub mod __internal {
    use magic_crypt::{MagicCrypt256, MagicCryptTrait};

    pub fn decrypt(key: &str, iv: &str, encrypted_value: &[u8]) -> String {
        let magic = MagicCrypt256::new(key, Some(iv));
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
