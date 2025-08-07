use std::fs::write;
use tempfile::tempdir;
mod test_helpers;
use test_helpers::run_dovetail_command;

const SAMPLE_YAML: &str = "
dev:
  run:
    - echo Hello World
";

const SAMPLE_YAML_DEPENDENCIES: &str = "
dev.1:
  run:
    - echo 'Going 1st'

dev.2:
  depends:
    - dev.1
  run:
    - echo 'Going 2nd'

dev.3:
  depends:
    - dev.2
  run:
    - echo 'Going 3rd'
";

#[test]
fn test_run_auto_command() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML).unwrap();

    let output = run_dovetail_command(&["run", "dev", "-y"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("The following commands will be run"));
    assert!(stdout.contains("echo Hello World"));
}

#[test]
fn test_run_prompt_command() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML).unwrap();

    let output = run_dovetail_command(&["run", "dev"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("y/N"));
}

#[test]
fn test_dependency_checker() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML_DEPENDENCIES).unwrap();

    let output = run_dovetail_command(&["run", "dev.3", "-y"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("The following commands will be run:"));
    assert!(stdout.contains("dev.1: echo 'Going 1st'"));
    assert!(stdout.contains("dev.2: echo 'Going 2nd'"));
    assert!(stdout.contains("dev.3: echo 'Going 3rd'"));
}
