#[test]
fn panic_messages() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fixtures/*.rs")
}
