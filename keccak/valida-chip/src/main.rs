use std::io::Write;

use sha3::{Digest, Keccak256};

fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

pub fn main() {
    let input: &[u8] = &[5u8; 32];
    let output = keccak(input);
    let output_hex = hex::encode(output);

    std::io::stdout().write_all(output_hex.as_bytes()).unwrap();
}
