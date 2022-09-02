# envcrypt

Drop-in replacements for [`env!`] and [`option_env!`]
that encrypt your variables at compile-time and decrypt them at runtime.

While it's still possible to reverse-engineer the values, `envcrypt` prevents
`strings <my-binary>` from trivially finding embedded secrets.

Since the secret must be decrypted at runtime,
`envcrypt!` returns an owned [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
instead of an string literal. Its API otherwise mirrors [`env!`] and [`option_env!`].

## Usage

As a replacement for [`env!`]

```rust
use envcrypt::envcrypt;

let my_super_secret_key: String = envcrypt!("SECRET_KEY");
// ...do stuff with your secret key
```

As a replacement for [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html)

```rust
use envcrypt::option_envcrypt;

if let Some(optional_value) = option_envcrypt!("OPTIONAL_SECRET_KEY") {
  // ...do stuff
}
```

With [`dotenvy`](https://crates.io/crates/dotenvy):

`.env`:

```dotenv
CLIENT_SECRET="my_client_secret"
SOME_TOKEN="some_token"
```

`build.rs`:

```rust
use dotenvy::dotenv_iter;

fn main(){
 println!("cargo:rerun-if-changed=.env");

 for item in dotenv_iter().unwrap() {
   let (key, value) = item.unwrap();
   println!("cargo:rustc-env=${key}=${value}");
 }

}
```

`main.rs`:

```rust
use envcrypt::envcrypt;

let client_secret: String = envcrypt!("CLIENT_SECRET");
```

## Details

Encryption is powered by [`magic_crypt`] using AES-256 encryption.

Inspired by [`litcrypt`]

License: MIT OR Apache-2.0
