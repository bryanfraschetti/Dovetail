// dovetail run <env>
use serde_yaml::Value;
use std::io::{self, Write};
use std::process;

pub fn run(yaml: &Value, environment: &String, skip_prompt: bool) {
    match &yaml[environment] {
        Value::Mapping(map) => {
            match map.get(&Value::String("run".to_string())) {
                Some(Value::Sequence(cmds)) => {
                    println!("The following commands will be run in '{}':", environment);
                    for cmd_val in cmds {
                        if let Value::String(cmd_str) = cmd_val {
                            println!("  {}", cmd_str);
                        }
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

                    for cmd_val in cmds {
                        if let Value::String(cmd_str) = cmd_val {
                            println!("Running: {}", cmd_str);
                            let status = process::Command::new("bash")
                                .arg("-c")
                                .arg(cmd_str)
                                .status();

                            match status {
                                Ok(s) if s.success() => {},
                                Ok(s) => eprintln!("Command exited with status: {}", s),
                                Err(e) => {
                                    eprintln!("Failed to execute '{}': {}", cmd_str, e);
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => eprintln!("No 'run' section found or it's not a list."),
            }
        }
        Value::Null => eprintln!("Environment '{}' not found.", environment),
        _ => eprintln!("Invalid format for environment '{}'", environment),
    }
}
