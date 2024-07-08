use methods::{FIB_ELF, FIB_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    
    let input: u32 = 46; // set to 25 or 46 for this benchmark
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    
    // Use the default prover
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary
    let prove_info = prover.prove(env, FIB_ELF).expect("Proving should succeed");

    // Extract journal of receipt
    let output: u32 = prove_info.receipt.journal.decode().expect("Journal should decode");

    // Verify the receipt
    prove_info.receipt.verify(FIB_ID).expect("Receipt should be verified");

    println!("Hello, world! I generated a proof of guest execution! {} is a public output from journal", output);
}