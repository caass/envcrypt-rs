use envcrypt::envcrypt;

fn main() {
    println!("{}", envcrypt!("ENCRYPTED_KEY"));
    println!("{}", env!("NAKED_KEY"));
}
