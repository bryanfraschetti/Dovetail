use std::fs::write;
use tempfile::tempdir;

mod test_helpers;
use test_helpers::run_dovetail_command;

#[test]
fn test_list_command() {
    let dir = tempdir().unwrap();
    let ini_path = dir.path().join("dovetail.ini");
    write(&ini_path, "[dev]\nkey = value").unwrap();

    let output = run_dovetail_command(&["list"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("dev"));
}
