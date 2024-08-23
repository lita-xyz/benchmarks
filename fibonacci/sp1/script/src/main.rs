//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use std::env;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <n>", args[0]);
        std::process::exit(1);
    }

    let n: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input. Please provide a valid u32 number.");
            std::process::exit(1);
        }
    };

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");

    // Read output.
    let a = proof.public_values.read::<u128>();
    let b = proof.public_values.read::<u128>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}