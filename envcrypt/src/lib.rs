//! Drop-in replacement for [`env!`](https://doc.rust-lang.org/std/macro.env.html)
//! that encrypts your variables at compile-time and decrypts them at runtime,
//! preventing naughty folks from snooping your binary for secrets or credentials.
//!
//! # Usage
//!
//! ```ignore
//! use envcrypt::envcrypt;
//!
//! fn main() {
//!   let my_super_secret_key = envcrypt!("SECRET_KEY");
//!   // do stuff with your secret key
//! }
//! ```
//!
//! With [`dotenv`](https://crates.io/crates/dotenv):
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
//! ```ignore
//! fn main() {
//!   println!("cargo:rerun-if-changed=.env");
//!
//!   for (key, value) in dotenv::vars() {
//!     println!("cargo:rustc-env=${key}=${value}");
//!   }
//! }
//! ```
//!
//! `main.rs`:
//!
//! ```ignore
//! use envcrypt::envcrypt;
//!
//! fn main() {
//!   let client_secret = envcrypt!("CLIENT_SECRET");
//! }
//! ```
//!
//! # Details
//!
//! Encryption is powered by [`MagicCrypt`](https://crates.io/crates/magic-crypt) using AES-256 encryption.

#[doc(hidden)]
pub mod __internal {
    use magic_crypt::{MagicCrypt256, MagicCryptTrait};

    #[doc(hidden)]
    pub unsafe fn decrypt(key: &str, iv: &str, encrypted_value: &[u8]) -> String {
        let magic = MagicCrypt256::new(key, Some(iv));
        let decrypted = magic.decrypt_bytes_to_bytes(encrypted_value).unwrap();
        String::from_utf8_unchecked(decrypted)
    }
}

/// Encrypt an environment variable at compile time,
/// and decrypt it at runtime.
///
/// ```ignore
/// let my_secret = envcrypt!("MY_SECRET");
/// let uh_oh = envcrypt!("NOT_FOUND", "This variable wasn't found!")
pub use envcrypt_macro::envcrypt;
