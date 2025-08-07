use std::fs::write;
use tempfile::tempdir;

mod test_helpers;
use test_helpers::run_dovetail_command;

const SAMPLE_YAML: &str = "
dev:
  key: value

staging:
  key: value
";

#[test]
fn test_list_command() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML).unwrap();

    let output = run_dovetail_command(&["list"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("dev"));
    assert!(stdout.contains("staging"));
}
