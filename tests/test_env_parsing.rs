use dovetail::subcmds::helpers::get_env_vars;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::write;
use tempfile::tempdir;

mod test_helpers;
use test_helpers::run_dovetail_command;

const SAMPLE_YAML: &str = "
dev:
  env:
    key1: val1

dev2:
  run:
    - echo test
";

const SAMPLE_YAML_2: &str = "
dev:
  depends:
    - dev_dependency
  env:
    key1: val1
    key2: val2
  run:
    - echo key1=$key1
    - echo key2=$key2
    - echo key3=$key3

dev_dependency:
  env:
    key1: notval1
    key3: val3
  run:
    - echo key1=$key1
    - echo key2=$key2
    - echo key3=$key3
";

#[test]
fn test_env_parsing() {
    let yaml_value: Value =
        serde_yaml::from_str(SAMPLE_YAML).expect("Failed to parse YAML");

    let hashmap1 = get_env_vars(&yaml_value, "dev");
    let mut expected_hashmap1 = HashMap::new();
    expected_hashmap1.insert("key1".to_string(), "val1".to_string());
    assert_eq!(hashmap1, expected_hashmap1);

    let hashmap2 = get_env_vars(&yaml_value, "dev2");
    let expected_hashmap2 = HashMap::new();
    assert_eq!(hashmap2, expected_hashmap2);
}

#[test]
fn test_independent_env_parsing() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("dovetail.yaml");
    write(&yaml_path, SAMPLE_YAML_2).unwrap();

    let output = run_dovetail_command(&["run", "dev", "-y"], &dir);

    if !output.status.success() {
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("key1=notval1"));
    assert!(stdout.contains("key2="));
    assert!(stdout.contains("key3=val3"));
    assert!(stdout.contains("key1=val1"));
    assert!(stdout.contains("key2=val2"));
    assert!(stdout.contains("key3="));
}
