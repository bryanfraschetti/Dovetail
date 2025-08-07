use std::fs::write;
use tempfile::tempdir;
mod test_helpers;
use test_helpers::run_dovetail_command;

const SAMPLE_YAML: &str = "
dev:
  release:
    git:
      run:
        - git push
";

const SAMPLE_YAML_DEPENDENCIES: &str = "
dev.debuild:
  run:
    - echo debuild -us -uc

prod:
  release:
    debian:
      depends:
        - dev.debuild
      run:
        - echo dput ppa
";

#[test]
fn test_run_release_command() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML).unwrap();

    let output = run_dovetail_command(&["release", "dev", "git", "-y"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("The following commands will be run:"));
    assert!(stdout.contains("git push"));
}

#[test]
fn test_release_with_nested_dependency() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML_DEPENDENCIES).unwrap();

    let output =
        run_dovetail_command(&["release", "prod", "debian", "-y"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    println!("{stdout:?}");
    assert!(stdout.contains("The following commands will be run:"));
    assert!(stdout.contains("dev.debuild: echo debuild -us -uc"));
    assert!(stdout.contains("debian: echo dput ppa"));
}
