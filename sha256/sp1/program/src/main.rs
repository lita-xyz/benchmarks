//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

pub fn main() {
    let input: &[u8] = &[5u8; 32];
    let output = sha2(input);
    let output_hex = hex::encode(output);
    
    sp1_zkvm::io::commit(&output_hex);
}
