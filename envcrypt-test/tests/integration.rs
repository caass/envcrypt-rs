use std::process::Output;

mod util;
use util::{include_binary_bytes, run_binary};

#[test]
fn error_messages_match_std_macros() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fixtures/error-message*.rs")
}

#[test]
fn encrypted_variables_are_decrypted_at_runtime() {
    let Output {
        status,
        stderr,
        stdout,
    } = run_binary();

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
}

#[test]
fn encrypted_variables_are_encrypted_at_compile_time() {
    let binary_text = String::from_utf8_lossy(include_binary_bytes!());

    assert!(!binary_text.contains("ENCRYPTED_VALUE"));
    assert!(binary_text.contains("NAKED_VALUE"));
}
