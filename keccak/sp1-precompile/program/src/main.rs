//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha3::{Digest, Keccak256};

#[sp1_derive::cycle_tracker]
fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

#[sp1_derive::cycle_tracker]
pub fn main() {
    let input: &[u8] = &[5u8; 32];
    let output = keccak(input);
    let output_hex = hex::encode(output);

    sp1_zkvm::io::commit(&output_hex);
}
