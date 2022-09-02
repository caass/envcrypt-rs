# envcrypt

[![envcrypt on crates.io](https://img.shields.io/crates/v/envcrypt)](https://crates.io/crates/envcrypt) [![envcrypt on docs.rs](https://docs.rs/envcrypt/badge.svg)](https://docs.rs/envcrypt) [![envcrypt on deps.rs](https://deps.rs/repo/github/caass/envcrypt/status.svg)](https://deps.rs/repo/github/caass/envcrypt)

Drop-in replacements for [`env!`](https://doc.rust-lang.org/std/macro.env.html) and [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html)
that encrypt your variables at compile-time and decrypt them at runtime.

While it's still possible to reverse-engineer the values, `envcrypt` prevents
`strings <my-binary>` from trivially finding embedded secrets.

Since the secret must be decrypted at runtime,
[`envc!`](https://docs.rs/envcrypt/latest/envcrypt/macro.envc.html) and [`option_envc!`](https://docs.rs/envcrypt/latest/envcrypt/macro.option_envc.html) return an owned [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
instead of `&'static str`. The API otherwise mirrors [`env!`](https://doc.rust-lang.org/std/macro.env.html) and [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html).

## Usage

The [`envc!`](https://docs.rs/envcrypt/latest/envcrypt/macro.envc.html) and [`option_envc!`](https://docs.rs/envcrypt/latest/envcrypt/macro.option_envc.html) macros can be used as drop-in replacements for [`env!`](https://doc.rust-lang.org/std/macro.env.html) and [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html), respectively.

### As a replacement for [`env!`](https://doc.rust-lang.org/std/macro.env.html)

```rust
use envcrypt::envc;

let my_super_secret_key: String = envc!("SECRET_KEY");
// ...do stuff with your secret key
```

### As a replacement for [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html)

```rust
use envcrypt::option_envc;

if let Some(optional_value) = option_envc!("OPTIONAL_SECRET_KEY") {
  // ...do stuff
}
```

### With [`dotenvy`](https://crates.io/crates/dotenvy):

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
use envcrypt::envc;

let client_secret: String = envc!("CLIENT_SECRET");
```

## Details

Encryption is powered by [`magic_crypt`](https://crates.io/crates/magic-crypt) using AES-256 encryption. `envcrypt` encrypts an environment variable, and then embeds the encrypted variable along with the encryption key and initialization vector in your binary at runtime.

You can check for yourself that your secrets are not visible in the binary by running `strings` on the compiled output:

```text
$ cat envcrypt-test/src/main.rs

use envcrypt::envc;

fn main() {
    println!("{}", envc!("ENCRYPTED_KEY"));
    println!("{}", env!("NAKED_KEY"));
}

$ cat envcrypt-test/build.rs

fn main() {
    println!("cargo:rustc-env=ENCRYPTED_KEY=ENCRYPTED_VALUE");
    println!("cargo:rustc-env=NAKED_KEY=NAKED_VALUE");
}

$ cargo build -p envcrypt-test
   Compiling envcrypt v0.2.0 (path/to/envcrypt)
   Compiling envcrypt-test v0.0.0 (path/to/envcrypt/envcrypt-test)
    Finished dev [unoptimized + debuginfo] target(s) in 1.73s


$ strings - target/debug/envcrypt-test | rg VALUE
NAKED_VALUE
```

Here are instructions for running `strings` yourself on [MacOS](https://www.unix.com/man-page/osx/1/strings/), [Linux](https://linux.die.net/man/1/strings), and [Windows](https://docs.microsoft.com/en-us/sysinternals/downloads/strings).

Inspired by [`litcrypt`](https://crates.io/crates/litcrypt), which I would have used except I want to open-source my code.
