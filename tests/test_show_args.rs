use std::fs::write;
use tempfile::tempdir;
mod test_helpers;
use test_helpers::run_dovetail_command;

#[test]
fn test_show_command() {
    let dir = tempdir().unwrap();
    let ini_path = dir.path().join("dovetail.ini");
    write(&ini_path, "[dev]\nkey = value\n").unwrap();

    let output = run_dovetail_command(&["show"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("[dev]\nkey = value\n"));
}

#[test]
fn test_show_environment_command() {
    let dir = tempdir().unwrap();
    let ini_path = dir.path().join("dovetail.ini");
    write(&ini_path, "[dev1]\nkey1 = value1\n\n[dev2]\nkey2 = value2").unwrap();

    let output = run_dovetail_command(&["show", "dev1"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("[dev1]\nkey1 = value1\n"));
    assert!(!stdout.contains("[dev2]\nkey2 = value2\n"));
}
