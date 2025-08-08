// dovetail release <env> <platform>
use crate::subcmds::run;
use serde_yaml::Value;
use std::collections::HashSet;

// Releases a platform for the specified environment
pub fn release(
    yaml: &Value,
    environment: &String,
    platform: &String,
    skip_prompt: bool,
) {
    let platform_map = yaml
        .get(environment)
        .and_then(|v| v.get("release"))
        // .and_then(|v| v.get(platform))
        .expect("Platform or Environment not found");

    let mut visited = HashSet::new();
    run::run_env_dfs(yaml, platform, platform_map, &mut visited, skip_prompt);
}
