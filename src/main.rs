use clap::{arg, Arg, Command};
use serde_yaml::Value;
use std::fs;
mod subcmds;
use subcmds::{show, list, run};

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

    if let Some(show_matches) = matches.subcommand_matches("show") {
        let env = show_matches.get_one::<String>("ENVIRONMENT");
        show::run(&yaml, env);
    }

    if let Some(_list) = matches.subcommand_matches("list") {
        list::run(&yaml);
    }

    if let Some(run_matches) = matches.subcommand_matches("run") {
        let env = run_matches.get_one::<String>("ENVIRONMENT").unwrap();
        let skip_prompt = run_matches.get_flag("yes");
        run::run(&yaml, env, skip_prompt);
    }
}
