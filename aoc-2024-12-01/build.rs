fn main() {
    cc::Build::new()
        .file("src/uthash_wrapper.c")
        .include("src")
        .opt_level(2)
        .compile("uthash_wrapper");

    println!("cargo:rerun-if-changed=src/uthash_wrapper.c");
    println!("cargo:rerun-if-changed=src/uthash_wrapper.h");
    println!("cargo:rerun-if-changed=src/uthash.h");
}
