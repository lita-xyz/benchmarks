use ark_std::{end_timer, start_timer};

pub fn main() {
    let (prove_main, verify_main) = guest::build_guest_main();

    let t_prove = start_timer!(|| "Jolt native: Keccak prove");
    let (output, proof) = prove_main();
    end_timer!(t_prove);

    let t_verify = start_timer!(|| "Jolt native: Keccak verify");
    let is_valid = verify_main(proof);
    end_timer!(t_verify);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
