use std::process::Command;

const TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "aarch64-pc-windows-msvc",
    "wasm32-unknown-unknown",
];

#[test]
fn compile_platforms() {
    let mut failures = Vec::new();
    let mut successes = Vec::new();

    for target in TARGETS {
        let status = Command::new("cargo")
            .args(["check", "--target", target])
            .status();
        match status {
            Ok(exit_status) if exit_status.success() => {
                successes.push(*target);
                println!("✅ Target [{}] compiled successfully.", target);
            }
            Ok(exit_status) => {
                failures.push(format!("{} (Exit code: {})", target, exit_status));
                println!("❌ Target [{}] FAILED compilation.", target);
            }
            Err(e) => {
                failures.push(format!("{} (Failed to run cargo: {})", target, e));
                println!("⚠️ Target [{}] could not execute cargo check.", target);
            }
        }
    }

    for failure in &failures {
        println!("{}", failure);
    }
    assert!(failures.len() == 0, "all platforms compiled successfully");
}
