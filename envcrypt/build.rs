fn main() {
    println!("cargo:rustc-env=SECRET_KEY=SOME_FAKE_SECRET_KEY_FOR_DOCTESTS");
    println!("cargo:rustc-env=CLIENT_SECRET=ANOTHER_SECRET_FOR_DOCTESTS");
}
