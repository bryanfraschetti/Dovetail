// dovetail show [<env>]
use serde_yaml::Value;
use std::fs;
use std::process;

pub fn run(yaml: &Value, environment: Option<&String>) {
    if let Some(env) = environment {
        match &yaml[env] {
            Value::Null => eprintln!("Environment '{}' not found.", env),
            env_val => {
                let fragment = serde_yaml::to_string(env_val).expect("Failed to format YAML");
                print!("{}:\n{}", env, fragment);
            }
        }
    } else {
        let content = match fs::read_to_string("dovetail.yaml") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read dovetail.yaml: {}", e);
                process::exit(1);
            }
        };
        println!("{}", content);
    }
}
