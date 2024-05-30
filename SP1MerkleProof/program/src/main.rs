//! A merkle proof verification program to be proven inside the zkVM. 
//! The leaves are 32-byte strings and the hash function is SHA-256.

#![no_main]
sp1_zkvm::entrypoint!(main);

use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof};
use sha2::{Digest};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

pub fn main() {

    let proof_hashes: Vec<[u8; 32]> = vec![
        [
            46, 125, 44, 3, 169, 80, 122, 226, 101, 236, 245, 181, 53, 104, 133, 165, 51, 147,
            162, 2, 157, 36, 19, 148, 153, 114, 101, 161, 162, 90, 239, 198
        ],
        [
            37, 47, 16, 200, 54, 16, 235, 202, 26, 5, 156, 11, 174, 130, 85, 235, 162, 249, 91,
            228, 209, 215, 188, 250, 137, 215, 36, 138, 130, 217, 241, 17
        ],
        [
            229, 160, 31, 238, 20, 224, 237, 92, 72, 113, 79, 34, 24, 15, 37, 173, 131, 101,
            181, 63, 151, 121, 247, 157, 196, 163, 215, 233, 57, 99, 249, 74
        ],
    ];
    let proof_hashes_copy = proof_hashes.clone();

    let proof = MerkleProof::<Sha256>::new(proof_hashes_copy);
    assert_eq!(proof.proof_hashes(), &proof_hashes);

    let input: &[u8] = &[5u8; 32];
    let output = sha2(input);
    let output_hex = hex::encode(output);
    
    sp1_zkvm::io::commit(&output_hex);
}
