#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
use sha2::Digest;

#[jolt::provable]
fn merkle_tree_proof() -> bool {
    let a = "Unpleasant astonished an diminu".as_bytes();
    let a_array = <[u8; 32]>::try_from(a).unwrap();
    let b = Sha256::hash(a);
    let concat_a_b = [a,&b[..]].concat();
    let c = Sha256::hash(&concat_a_b);
    let d = Sha256::hash(&c[..]);
    let concat_c_d = [&c[..],&d[..]].concat();
    let e_root = Sha256::hash(&concat_c_d);
    let leaves = [a_array, b, c, d, e_root];

    // The merkle tree looks like this:
    //     e (root)
    //    / \
    //   c   d
    //  / \
    // a   b
    let merkle_tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
    let indices_to_prove = vec![0, 1, 2, 3];
    let leaves_to_prove = leaves.get(0..4).unwrap();

    let leaves_merkle = merkle_tree.proof(&indices_to_prove);
    let root = merkle_tree.root().unwrap();
    
    let root_leaves_merkle = leaves_merkle.verify(root, &indices_to_prove, leaves_to_prove, leaves.len());

    root_leaves_merkle

}