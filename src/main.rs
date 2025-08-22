use clap::{Arg, Command, arg};
use serde_yaml::Value;
use std::fs;
mod subcmds;
use subcmds::{list, release, run, show};

fn main() {
    let cmd = Command::new("dovetail")
        .version("0.1.0")
        .about("The project agnostic workflow manager")
        .arg_required_else_help(true) // This ensures help is shown if no subcommand is given
        .subcommand(
            Command::new("show")
                .about("Displays the contents of the dovetail.yaml file or a specific environment")
                .arg(arg!(<ENVIRONMENT> "The name of the environment").required(false)),
        )
        .subcommand(Command::new("list").about("Lists dovetail.yaml environments"))
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
        .subcommand(
            Command::new("release")
                .about("Executes the release section of an environment")
                .arg(arg!(<ENVIRONMENT> "The name of the environment"))
                .arg(arg!(<PLATFORM> "The name of the release platform"))
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .required(false)
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("clean")
                .about("Alias for `run clean`. Executes the clean environment")
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .action(clap::ArgAction::SetTrue),
                )
        );

    let matches = cmd.get_matches();

    let content = match fs::read_to_string("dovetail.yaml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read dovetail.yaml: {e}");
            std::process::exit(1);
        }
    };

    let yaml: Value = match serde_yaml::from_str(&content) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("Failed to parse YAML: {e}");
            std::process::exit(1);
        }
    };

    match matches.subcommand() {
        Some(("show", show_matches)) => {
            let env = show_matches.get_one::<String>("ENVIRONMENT");
            show::show(&yaml, env);
        }
        Some(("list", _)) => {
            list::list(&yaml);
        }
        Some(("run", run_matches)) => {
            let env = run_matches.get_one::<String>("ENVIRONMENT").unwrap();
            let skip_prompt = run_matches.get_flag("yes");
            run::run(&yaml, env, skip_prompt);
        }
        Some(("clean", clean_matches)) => {
            let skip_prompt = clean_matches.get_flag("yes");
            run::run(&yaml, &"clean".to_string(), skip_prompt);
        }
        Some(("release", release_matches)) => {
            let env = release_matches.get_one::<String>("ENVIRONMENT").unwrap();
            let platform =
                release_matches.get_one::<String>("PLATFORM").unwrap();
            let skip_prompt = release_matches.get_flag("yes");
            release::release(&yaml, env, platform, skip_prompt);
        }
        _ => {} // No need for this case because of arg_required_else_help
    }
}
