#!/usr/bin/env python3
"""
Generate Python bindings for the Rust library using UniFFI.
This script builds the library and generates Python bindings.
"""

import subprocess
import sys
from pathlib import Path

def main():
    # Paths
    project_root = Path(__file__).parent.parent
    udl_file = project_root / "src" / "aoc_ffi_day01.udl"
    lib_file = project_root / "target" / "release" / "libaoc_ffi_day01.so"
    output_dir = project_root / "bindings" / "python"

    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)

    # Build the library first
    print("Building Rust library...")
    result = subprocess.run(
        ["cargo", "build", "--release", "--lib"],
        cwd=project_root,
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print(f"Error building library:\n{result.stderr}")
        sys.exit(1)

    print(f"✓ Library built successfully")

    # Generate Python bindings using uniffi-bindgen
    print(f"\nGenerating Python bindings...")
    print(f"  UDL: {udl_file}")
    print(f"  Library: {lib_file}")
    print(f"  Output: {output_dir}")

    # Use cargo to run uniffi-bindgen
    result = subprocess.run([
        "cargo", "run",
        "--bin", "uniffi-bindgen",
        "generate",
        str(udl_file),
        "--library", str(lib_file),
        "--language", "python",
        "--out-dir", str(output_dir)
    ], capture_output=True, text=True, cwd=project_root)

    if result.returncode != 0:
        print(f"\nError generating bindings:\n{result.stderr}")
        print("\nTrying alternative method with uniffi-bindgen command...")

        # Try direct uniffi-bindgen command
        result = subprocess.run([
            "uniffi-bindgen",
            "generate",
            str(udl_file),
            "--library", str(lib_file),
            "--language", "python",
            "--out-dir", str(output_dir)
        ], capture_output=True, text=True)

        if result.returncode != 0:
            print(f"\nError: {result.stderr}")
            print("\nPlease install uniffi-bindgen:")
            print("  cargo install uniffi-bindgen --version 0.28.3")
            sys.exit(1)

    print(f"\n✓ Python bindings generated in {output_dir}/")
    print(f"\nGenerated files:")
    for f in sorted(output_dir.iterdir()):
        print(f"  - {f.name}")

    print(f"\nTo use the bindings, add this to your Python path:")
    print(f"  export PYTHONPATH={output_dir}:$PYTHONPATH")
    print(f"\nOr in Python:")
    print(f"  import sys")
    print(f"  sys.path.insert(0, '{output_dir}')")

if __name__ == "__main__":
    main()
