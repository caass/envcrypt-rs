# envcrypt ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![envcrypt on crates.io](https://img.shields.io/crates/v/envcrypt)](https://crates.io/crates/envcrypt) [![envcrypt on docs.rs](https://docs.rs/envcrypt/badge.svg)](https://docs.rs/envcrypt) [![envcrypt on deps.rs](https://deps.rs/repo/github/caass/envcrypt/status.svg)](https://deps.rs/repo/github/caass/envcrypt)

Drop-in replacements for [`env!`][__link0] and [`option_env!`][__link1] that encrypt your variables at compile-time and decrypt them at runtime.

While it’s still possible to reverse-engineer the values, `envcrypt` prevents `strings <my-binary>` from trivially finding embedded secrets.

Since the secret must be decrypted at runtime, `envcrypt!` returns an owned [`String`][__link2] instead of an string literal. Its API otherwise mirrors [`env!`][__link3] and [`option_env!`][__link4].


## Usage

As a replacement for [`env!`][__link5]


```rust
use envcrypt::envcrypt;

let my_super_secret_key: String = envcrypt!("SECRET_KEY");
// ...do stuff with your secret key
```

As a replacement for [`option_env!`][__link6]


```rust
use envcrypt::option_envcrypt;

if let Some(optional_value) = option_envcrypt!("OPTIONAL_SECRET_KEY") {
  // ...do stuff
}
```

With [`dotenvy`][__link7]:

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

Encryption is powered by [`magic_crypt`][__link8] using AES-256 encryption.

Inspired by [`litcrypt`][__link9]


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG_Smw7rergJMG1TyXaClBPNHG55jhoOp6-kUG_KLHyCglqgdYXKEG99Gb22igwziG_TkV_QX9vlGG13p6xw5JJcBG4JPiMqSo_7vYWSBg2ttYWdpYy1jcnlwdGYzLjEuMTBrbWFnaWNfY3J5cHQ
 [__link0]: `env!`
 [__link1]: `option_env!`
 [__link2]: https://doc.rust-lang.org/std/string/struct.String.html
 [__link3]: `env!`
 [__link4]: `option_env!`
 [__link5]: `env!`
 [__link6]: https://doc.rust-lang.org/std/macro.option_env.html
 [__link7]: https://crates.io/crates/dotenvy
 [__link8]: https://crates.io/crates/magic-crypt/3.1.10
 [__link9]: https://docs.rs/crate/litcrypt/0.3.0
