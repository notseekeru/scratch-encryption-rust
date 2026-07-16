use sha2::{ Digest, Sha256 };

fn main() {
    let input = "hello world";

    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();

    let hash_string = format!("{:x}", result);

    println!("SHA-256: {}", hash_string);
    // Output: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
}
