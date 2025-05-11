// dovetail list
use serde_yaml::Value;

pub fn run(yaml: &Value) {
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
