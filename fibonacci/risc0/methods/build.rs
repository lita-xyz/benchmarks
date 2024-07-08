fn main() {
    // Rerun if any file in the methods/guest/src directory changes
    println!("cargo:rerun-if-changed=guest/src");
    
    // Rerun if the Cargo.toml file changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Rerun if this build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Call the risc0 build function
    risc0_build::embed_methods();
}