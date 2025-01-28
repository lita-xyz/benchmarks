#![no_main]

use sha3::{Digest, Keccak256};

fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

valida_rs::entrypoint!(main);

pub fn main() {
    let input: &[u8] = &[5u8; 32];
    let output = keccak(input);

    println!("{:?}", &output);
}
