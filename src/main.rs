use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    println!("generated password: {}", password)
}