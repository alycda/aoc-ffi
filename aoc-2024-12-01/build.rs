fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/main.rs", [&path])
        .build()?;

    // Add C++ standard library and optimization flags
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-O2")  // Add optimization to suppress _FORTIFY_SOURCE warning
        .flag_if_supported("-Wno-cpp")  // Suppress preprocessor warnings
        .compile("aoc-cpp-bindings");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cpp_sort.h");

    Ok(())
}
