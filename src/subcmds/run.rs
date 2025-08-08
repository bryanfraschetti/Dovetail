// dovetail run <env>
use serde_yaml::Value;
use std::collections::HashSet;

use crate::subcmds::helpers;

pub fn run(yaml: &Value, environment: &String, skip_prompt: bool) {
    let mut visited = HashSet::new();
    run_env_dfs(yaml, environment, yaml, &mut visited, skip_prompt);
}

pub fn run_env_dfs(
    yaml: &Value,
    environment: &String,
    dep_res_scope: &serde_yaml::Value,
    visited: &mut HashSet<String>,
    skip_prompt: bool,
) {
    if visited.contains(environment) {
        return;
    }
    visited.insert(environment.clone());

    // Traverse dependencies depth first
    if let Some(Value::Mapping(map)) = &dep_res_scope.get(environment)
        && let Some(Value::Sequence(depends)) =
            map.get(Value::String("depends".to_string()))
    {
        for dep in depends {
            if let Value::String(dep_env) = dep {
                run_env_dfs(yaml, dep_env, yaml, visited, skip_prompt);
            }
        }
    }

    let env_vars = helpers::get_env_vars(dep_res_scope, environment);
    let env_commands = helpers::get_env_cmds(dep_res_scope, environment);
    helpers::execute(env_commands, &env_vars, skip_prompt);
}
