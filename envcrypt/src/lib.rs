#![deny(missing_docs, clippy::missing_docs_in_private_items)]
#![cfg_attr(windows, doc = include_str!("..\\README.md"))]
#![cfg_attr(not(windows), doc = include_str!("../README.md"))]

mod decrypt;

/// For internal use only.
#[doc(hidden)]
pub mod __internal {
    #[doc(hidden)]
    pub use super::decrypt::decrypt;
}

/// Inspects and encrypts an environment variable at compile time
/// and decrypts at runtime.
///
/// This macro will expand to the value of the named environment variable at compile time,
/// yielding an expression of type `&'static str`.
/// Use [`std::env::var`] instead if you want to read the value at runtime.
///
/// ```rust
/// # use envcrypt::envc;
/// let path: &'static str = envc!("PATH");
/// println!("the $PATH variable at the time of compiling was: {path}");
/// ```
///
///  If the environment variable is not defined, then a compilation error will be emitted.
/// To not emit a compile error, use the [`option_envc!`] macro instead.
///
/// ```compile_fail
/// # use envcrypt::envc;
/// let unlikely_variable: &'static str = envc!("HIGHLY_UNLIKELY_ENVIRONMENT_VARIABLE");
/// ```
#[doc(inline)]
pub use envcrypt_macro::envc;

/// Optionally inspects and encrypts an environment variable at compile time
/// and decrypts at runtime.
///
/// If the named environment variable is present at compile time,
/// this will expand into an expression of type `Option<&'static str>`
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
/// let key: Option<&'static str> = option_envc!("SECRET_KEY");
/// println!("the secret key might be: {key:?}");
/// ```
#[doc(inline)]
pub use envcrypt_macro::option_envc;
