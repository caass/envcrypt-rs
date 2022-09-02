use std::process::{Command, Output};

pub(crate) fn run_binary() -> Output {
    Command::new(env!("CARGO_BIN_EXE_envcrypt-test"))
        .env("RUST_BACKTRACE", "1")
        .output()
        .expect("Failed to execute test binary!")
}

macro_rules! include_binary_bytes {
    () => {
        include_bytes!(env!("CARGO_BIN_EXE_envcrypt-test"))
    };
}

pub(crate) use include_binary_bytes;
