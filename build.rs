use std::env;

use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = env::var("CARGO_MANIFEST_DIR")?;
    let output = PathBuf::from(env::var("OUT_DIR")?);
    let library = output.join("libcalculations.a");
    let status = Command::new("zig")
        .current_dir(&root)
        .args([
            "build-lib",
            "zig/calculations.zig",
            "-static",
            "-O",
            "ReleaseFast",
        ])
        .arg(format!(
                "-femit-bin={}",
                library.display()
        ))
        .status()?;
    assert!(status.success(), "Zig compilation failed");

    println!(
        "cargo:rustc-link-search=native={}",
        output.display()
    );

    println!("cargo:rustc-link-lib=static=calculations");

    println!("cargo:rerun-if-changed=zig/calculations.zig");
    Ok(())
}
