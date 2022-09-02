use envcrypt::envc;

fn main() {
    println!("{}", envc!("ENCRYPTED_KEY"));
    println!("{}", env!("NAKED_KEY"));
}
