//! Shorthand macros for generating code that will decrypt variables at runtime

/// Generate code that will call decrypt a variable at runtime
macro_rules! decrypt {
    ($key:ident, $value:ident, $nonce:ident) => {
        quote!({ ::envcrypt::__internal::decrypt(#$key, #$value, #$nonce) })
    };
}

/// Generate code that will optionally decrypt `Some` variable at runtime
macro_rules! decrypt_some {
    ($key:ident, $value:ident, $nonce:ident) => {{
        let output = decrypt!($key, $value, $nonce);
        quote!(::std::option::Option::Some(#output))
    }};
}

/// Generate code that will resolve to `None`
macro_rules! decrypt_none {
    () => {
        quote!(::core::option::Option::<&'static str>::None)
    };
}

pub(crate) use decrypt;
pub(crate) use decrypt_none;
pub(crate) use decrypt_some;
