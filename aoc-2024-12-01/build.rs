fn main() {
    cc::Build::new()
        .file("src/uthash_wrapper.c")
        .include("src")
        .opt_level(2)
        .compile("uthash_wrapper");

    // Generate UniFFI bindings
    uniffi::generate_scaffolding("src/aoc_ffi_day01.udl")
        .expect("Failed to generate UniFFI scaffolding");

    println!("cargo:rerun-if-changed=src/uthash_wrapper.c");
    println!("cargo:rerun-if-changed=src/uthash_wrapper.h");
    println!("cargo:rerun-if-changed=src/uthash.h");
    println!("cargo:rerun-if-changed=src/aoc_ffi_day01.udl");
}
