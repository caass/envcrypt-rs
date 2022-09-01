# envcrypt

Drop-in replacement for [`env!`](https://doc.rust-lang.org/std/macro.env.html)
that encrypts your variables at compile-time and decrypts them at runtime,
preventing naughty folks from snooping your binary for secrets or credentials.

Inspired by [`litcrypt`](https://crates.io/crates/litcrypt).

## Usage

```rust
use envcrypt::envcrypt;

fn main() {
  let my_super_secret_key = envcrypt!("SECRET_KEY");
  // do stuff with your secret key
}
```

With [`dotenv`](https://crates.io/crates/dotenv):

`.env`:

```dotenv
CLIENT_SECRET="my_client_secret"
SOME_TOKEN="some_token"
```

`build.rs`:

```rust
fn main() {
  println!("cargo:rerun-if-changed=.env");

  for (key, value) in dotenv::vars();
    println!("cargo:rustc-env=${key}=${value}");
}
```

`main.rs`:

```rust
use envcrypt::envcrypt;

fn main() {
  let client_secret = envcrypt!("CLIENT_SECRET");
}
```

## Details

Encryption is powered by [`MagicCrypt`](https://crates.io/crates/magic-crypt) using AES-256 encryption.
