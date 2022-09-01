use std::process::Command;

#[test]
fn check_binary() {
    let stdout = Command::new(env!("CARGO_BIN_EXE_envcrypt-test"))
        .output()
        .expect("Failed to execute test binary!")
        .stdout;
    let stdout_text = String::from_utf8_lossy(&stdout);

    assert!(stdout_text.contains("ENCRYPTED_VALUE"));
    assert!(stdout_text.contains("NAKED_VALUE"));

    let binary = include_bytes!(env!("CARGO_BIN_EXE_envcrypt-test"));
    let binary_text = String::from_utf8_lossy(binary);

    assert!(!binary_text.contains("ENCRYPTED_VALUE"));
    assert!(binary_text.contains("NAKED_VALUE"));
}
