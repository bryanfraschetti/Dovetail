use clap::{arg, Arg, Command};
use serde_yaml::Value;
use std::fs;
use std::process;
use std::io::{self, Write};

fn main() {
    let matches = Command::new("dovetail")
        .version("0.1.0")
        .about("The project agnostic workflow manager")
        .subcommand(
            Command::new("show")
                .about("Displays the contents of the dovetail.yaml file or a specific environment")
                .arg(arg!(<ENVIRONMENT> "The name of the environment").required(false)),
        )
        .subcommand(
            Command::new("list")
                .about("Lists dovetail.yaml environments")
        )
        .subcommand(
            Command::new("run")
                .about("Executes the run section of an environment")
                .arg(arg!(<ENVIRONMENT> "The name of the environment"))
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .required(false)
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();

    let content = match fs::read_to_string("dovetail.yaml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read dovetail.yaml: {}", e);
            std::process::exit(1);
        }
    };

    let yaml: Value = match serde_yaml::from_str(&content) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("Failed to parse YAML: {}", e);
            std::process::exit(1);
        }
    };

    // `dovetail show [<environment>]`
    if let Some(show_matches) = matches.subcommand_matches("show") {
        if let Some(env) = show_matches.get_one::<String>("ENVIRONMENT") {
            match &yaml[env] {
                Value::Null => eprintln!("Environment '{}' not found.", env),
                env_val => {
                    let fragment = serde_yaml::to_string(env_val).expect("Failed to format YAML");
                    print!("{}:\n{}", env, fragment);
                }
            }
        } else {
            println!("{}", content);
        }
    }

    // `dovetail list`
    if let Some(_list) = matches.subcommand_matches("list") {
        match &yaml {
            Value::Mapping(map) => {
                println!("Environments:");
                for (key, _) in map {
                    if let Value::String(env) = key {
                        println!("{}", env);
                    }
                }
            }
            _ => eprintln!("dovetail.yaml does not contain a top-level mapping."),
        }
    }

    // `dovetail run <environment>`
    if let Some(run_matches) = matches.subcommand_matches("run") {
        let env = run_matches.get_one::<String>("ENVIRONMENT").unwrap();
        let skip_prompt = run_matches.get_flag("yes");

        match &yaml[env] {
            Value::Mapping(map) => {
                match map.get(&Value::String("run".to_string())) {
                    Some(Value::Sequence(cmds)) => {
                        println!("The following commands will be run in '{}':", env);
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
            Value::Null => eprintln!("Environment '{}' not found.", env),
            _ => eprintln!("Invalid format for environment '{}'", env),
        }
    }
}
