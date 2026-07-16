use sha2::{ Sha256, Digest };

fn main() {
    hash("hello world");
}

fn hash(input: &str) {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());
    let manual_result = hasher.finalize();

    println!("Manual Hash:  {:x}\n", manual_result);
}
