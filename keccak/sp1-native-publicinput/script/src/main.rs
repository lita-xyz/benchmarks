//! A simple script to generate and verify the proof of a given program.

use ark_std::{end_timer, start_timer};
use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; 32];
    stdin.write(&input);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let t_prove = start_timer!(|| "SP1 native with public input and no hex: Keccak prove");
    let mut proof = client.prove(&pk, stdin).run().expect("proving failed");
    end_timer!(t_prove);

    // Read and verify the output.
    let hash = proof.public_values.read::<String>();

    println!("hash: {}", hash);

    // Verify proof.
    let t_verify = start_timer!(|| "SP1 native with public input and no hex: Keccak verify");
    client.verify(&proof, &vk).expect("verification failed");
    end_timer!(t_verify);

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
