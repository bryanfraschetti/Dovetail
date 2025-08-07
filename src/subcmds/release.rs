// dovetail release <env> <platform>
use crate::subcmds::helpers;
use serde_yaml::Value;
use std::collections::HashSet;

// Releases a platform for the specified environment
pub fn release(
    yaml: &Value,
    environment: &String,
    platform: &String,
    skip_prompt: bool,
) {
    // Validate environment and platform
    let env_map = yaml.get(environment).and_then(|v| v.get("release"));

    let platform_map = match env_map {
        Some(map) => map,
        None => {
            eprintln!(
                "Platform '{platform}' not found in release section of environment '{environment}'."
            );
            return;
        }
    };

    let mut visited = HashSet::new();

    // Collect first level dependencies and commands
    let direct_dependencies =
        helpers::collect_dependencies(platform_map, platform, &mut visited);
    // The declared dependencies won't be available in platform_map and thus
    // cannot be recursively searched. That could be solved by running
    // collect_dependencies on each dependency but at this time,
    // that won't be supported
    let dependency_commands =
        helpers::collect_commands_for_dependencies(yaml, &direct_dependencies);
    let env_vars = helpers::get_env_vars(yaml, environment);
    let env_commands = helpers::get_env_cmds(platform_map, platform);

    // Execute commands
    helpers::execute(dependency_commands, &env_vars, skip_prompt);
    helpers::execute(env_commands, &env_vars, skip_prompt);
}
