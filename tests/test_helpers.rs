use std::process::Command;
use std::path::PathBuf;

pub fn run_dovetail_command(args: &[&str], dir: &tempfile::TempDir) -> std::process::Output {
    let mut binary_path = PathBuf::from("target/debug/dovetail");
    binary_path = binary_path.canonicalize()
        .expect("Failed to find dovetail binary. Did you run `cargo build`?");

    Command::new(binary_path)
        .args(args)
        .current_dir(dir.path())
        .output()
        .expect("Failed to run dovetail")
}