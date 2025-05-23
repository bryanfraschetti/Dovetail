use std::process;
use serde_yaml::Value;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn execute(commands: Vec<(String, String)>, skip_prompt: bool){
    if commands.is_empty() {
        eprintln!("No commands to run.");
        return;
    }

    println!("The following commands will be run:");
    for (env, cmd) in &commands {
        println!("{}: {}", env, cmd);
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
        println!("Running in {}: {}", env, cmd);
        let status = process::Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(s) => eprintln!("Command in '{}' exited with status: {}", env, s),
            Err(e) => {
                eprintln!("Failed to execute '{}': {}", cmd, e);
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

    if let Value::Mapping(map) = &yaml[environment] {
        if let Some(Value::Sequence(depends)) = map.get(&Value::String("depends".to_string())) {
            for dep in depends {
                if let Value::String(dep_env) = dep {
                    if !visited.contains(dep_env) {
                        ordered_dependencies.extend(collect_dependencies(yaml, &dep_env, visited));
                        ordered_dependencies.push(dep_env.clone());
                    }
                }
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
        if let Value::Mapping(map) = &yaml[dep] {
            if let Some(Value::Sequence(cmds)) = map.get(&Value::String("run".to_string())) {
                for cmd in cmds {
                    if let Value::String(cmd_str) = cmd {
                        commands.push((dep.clone(), cmd_str.clone()));
                    }
                }
            }
        }
    }

    commands
}

pub fn get_env_cmds(
    yaml: &Value,
    env: &String
)->Vec<(String, String)> {
    let env_cmds = collect_commands_for_dependencies(&yaml, &[env.to_string()]);

    env_cmds
}