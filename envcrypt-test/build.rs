fn main() {
    println!("cargo:rustc-env=ENCRYPTED_KEY=ENCRYPTED_VALUE");
    println!("cargo:rustc-env=NAKED_KEY=NAKED_VALUE");
}
