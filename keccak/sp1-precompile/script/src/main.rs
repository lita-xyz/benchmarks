//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use ark_std::{end_timer, start_timer};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let stdin = SP1Stdin::new();
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let t_prove = start_timer!(|| "SP1 precompile: Keccak prove");
    let mut proof = client.prove(&pk, stdin).run().expect("proving failed");
    end_timer!(t_prove);
    
    // Read and verify the output.
    let hash = proof.public_values.read::<String>();

    println!("hash: {}", hash);
    
    // Verify proof.
    let t_verify = start_timer!(|| "SP1 precompile: Keccak verify");
    client.verify(&proof, &vk).expect("verification failed");
    end_timer!(t_verify);

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
