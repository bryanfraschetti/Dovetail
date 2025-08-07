use dovetail::subcmds::helpers::get_env_vars;
use serde_yaml::Value;
use std::collections::HashMap;

const SAMPLE_YAML: &str = "
dev:
  env:
    key1: val1

dev2:
  run:
    - echo test
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
