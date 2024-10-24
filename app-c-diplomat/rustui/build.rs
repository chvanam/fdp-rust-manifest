use std::process::Command;

fn main() {
    let output = Command::new("diplomat-tool")
        .args(["c", "c/"])
        .output()
        .expect("Failed to execute diplomat-tool");

    if !output.status.success() {
        panic!(
            "diplomat-tool failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
