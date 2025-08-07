use colored::*;
use serde_yaml::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write};
use std::process;

pub fn execute(
    commands: Vec<(String, String)>,
    env_vars: &HashMap<String, String>,
    skip_prompt: bool,
) {
    if commands.is_empty() {
        // eprintln!("No commands to run.");
        return;
    }

    println!("The following commands will be run:");
    for (env, cmd) in &commands {
        println!("{}: {}", env.green(), cmd);
    }

    if !skip_prompt {
        print!("Are you sure you want to run these commands? [y/N] ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.to_lowercase() != "y" {
            println!("Aborted.");
            return;
        }
    }

    for (env, cmd) in commands {
        println!("\n{}: {}", env.green(), cmd);
        let status = process::Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .envs(env_vars)
            .status();

        println!();
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => eprintln!(
                "Command {} in '{}' exited with status: {}",
                cmd.red(),
                env.red(),
                s.to_string().red()
            ),
            Err(e) => {
                eprintln!(
                    "Failed to execute '{}': {}",
                    cmd.red(),
                    e.to_string().red()
                );
                break;
            }
        }
    }
}

pub fn collect_dependencies(
    yaml: &Value,
    environment: &String,
    visited: &mut HashSet<String>,
) -> Vec<String> {
    if visited.contains(environment) {
        return vec![];
    }

    visited.insert(environment.clone());
    let mut ordered_dependencies = Vec::new();

    if let Value::Mapping(map) = &yaml[environment]
        && let Some(Value::Sequence(depends)) =
            map.get(Value::String("depends".to_string()))
    {
        for dep in depends {
            if let Value::String(dep_env) = dep
                && !visited.contains(dep_env)
            {
                ordered_dependencies
                    .extend(collect_dependencies(yaml, dep_env, visited));
                ordered_dependencies.push(dep_env.clone());
            }
        }
    }
    ordered_dependencies
}

// Collects commands for a list of dependencies
pub fn collect_commands_for_dependencies(
    yaml: &Value,
    dependencies: &[String],
) -> Vec<(String, String)> {
    let mut commands = Vec::new();

    for dep in dependencies {
        if let Value::Mapping(map) = &yaml[dep]
            && let Some(Value::Sequence(cmds)) =
                map.get(Value::String("run".to_string()))
        {
            for cmd in cmds {
                if let Value::String(cmd_str) = cmd {
                    commands.push((dep.clone(), cmd_str.clone()));
                }
            }
        }
    }

    commands
}

pub fn get_env_vars(yaml: &Value, env: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    if let Some(env_section) = yaml.get(env)
        && let Some(env_vars) = env_section.get("env")
        && let Some(mapping) = env_vars.as_mapping()
    {
        for (key, value) in mapping {
            if let (Some(k), Some(v)) = (key.as_str(), value.as_str()) {
                result.insert(k.to_string(), v.to_string());
            }
        }
    }

    result
}

pub fn get_env_cmds(yaml: &Value, env: &String) -> Vec<(String, String)> {
    collect_commands_for_dependencies(yaml, &[env.to_string()])
}
