use std::{
    io,
    process::{Command, Output},
};

fn run_binary() -> io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_envcrypt-test"))
        .env("RUST_BACKTRACE", "1")
        .output()
}

macro_rules! binary_bytes {
    () => {
        include_bytes!(env!("CARGO_BIN_EXE_envcrypt-test"))
    };
}

#[test]
fn check_binary() {
    let Output {
        status,
        stderr,
        stdout,
    } = run_binary().expect("Failed to execute test binary!");

    let stderr = String::from_utf8_lossy(&stderr);
    let stdout = String::from_utf8_lossy(&stdout);

    assert!(
        status.success(),
        "Test binary failed.\nstderr dump:\n{stderr}"
    );

    assert!(
        stdout.contains("ENCRYPTED_VALUE"),
        "Test binary failed.\nstderr dump:\n{stderr}"
    );
    assert!(
        stdout.contains("NAKED_VALUE"),
        "Test binary failed.\nstderr dump:\n{stderr}"
    );

    let binary_text = String::from_utf8_lossy(binary_bytes!());

    assert!(!binary_text.contains("ENCRYPTED_VALUE"));
    assert!(binary_text.contains("NAKED_VALUE"));
}
