// dovetail run <env>
use serde_yaml::Value;
use std::collections::HashSet;

use crate::subcmds::helpers;

pub fn run(yaml: &Value, environment: &String, skip_prompt: bool) {
    let mut visited = HashSet::new();

    let dependencies =
        helpers::collect_dependencies(yaml, environment, &mut visited);
    let dependency_commands =
        helpers::collect_commands_for_dependencies(yaml, &dependencies);
    let env_commands = helpers::get_env_cmds(yaml, environment);
    helpers::execute(dependency_commands, skip_prompt);
    helpers::execute(env_commands, skip_prompt);
}
