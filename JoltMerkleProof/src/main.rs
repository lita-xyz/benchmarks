use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
use sha2::Digest;

pub fn main() {
    let (prove_merkle_tree_proof, verify_merkle_tree_proof) = guest::build_merkle_tree_proof();


    // let indices_to_prove = vec![0, 1, 2, 3];
    // let leaves_to_prove = leaves.get(0..4).unwrap();

    // let leaves_merkle = merkle_tree.proof(&indices_to_prove);
    // let root = merkle_tree.root().unwrap();
    
    // let root_leaves_merkle = leaves_merkle.verify(root, &indices_to_prove, leaves_to_prove, leaves.len());

    let (output, proof) = prove_merkle_tree_proof;
    let is_valid = verify_merkle_tree_proof(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}