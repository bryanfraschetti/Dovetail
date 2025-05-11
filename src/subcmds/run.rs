// dovetail run <env>
use serde_yaml::Value;
use std::collections::HashSet;
use std::io::{self, Write};
use std::process;

pub fn run(yaml: &Value, environment: &String, skip_prompt: bool) {
    let mut visited = HashSet::new();
    let mut commands = Vec::new();

    collect_commands(yaml, environment, &mut visited, &mut commands);

    if commands.is_empty() {
        eprintln!("No commands to run in '{}'.", environment);
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

// Collects all commands for the target environment and its dependencies
fn collect_commands(
    yaml: &Value,
    environment: &String,
    visited: &mut HashSet<String>,
    commands: &mut Vec<(String, String)>,
) {
    if visited.contains(environment) {
        return;
    }
    visited.insert(environment.clone());

    if let Value::Mapping(map) = &yaml[environment] {
        // Check for dependencies and collect their commands first
        if let Some(Value::Sequence(depends)) = map.get(&Value::String("depends".to_string())) {
            println!("Environment '{}' has dependencies:", environment);
            for dep in depends {
                if let Value::String(dep_env) = dep {
                    println!("  - {}", dep_env);
                    collect_commands(yaml, &dep_env, visited, commands);
                }
            }
        }

        // Collect this environment's commands
        if let Some(Value::Sequence(cmds)) = map.get(&Value::String("run".to_string())) {
            for cmd_val in cmds {
                if let Value::String(cmd_str) = cmd_val {
                    commands.push((environment.clone(), cmd_str.clone()));
                }
            }
        }
    }
}
