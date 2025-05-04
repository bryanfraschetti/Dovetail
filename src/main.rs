use clap::{Command, arg};
use std::fs;
use ini::Ini;

fn main() {
    let matches = Command::new("dovetail")
        .version("0.1.0")
        .about("The project agnostic workflow manager")
        .subcommand(
            Command::new("show")
                .about("Displays the contents of the dovetail.ini file or a specific environment")
                .arg(arg!(<ENVIRONMENT> "The name of the environment").required(false))
        )
        .subcommand(
            Command::new("list")
                .about("Lists things from the dovetail.ini file")
                .subcommand(
                    Command::new("environments")
                        .about("Lists all environments defined in the dovetail.ini file")
                )
        )
        .get_matches();

    // `dovetail show [<environment>]`
    if let Some(show_matches) = matches.subcommand_matches("show") {
        if let Some(env) = show_matches.get_one::<String>("ENVIRONMENT") {
            // Show only that environment section
            match Ini::load_from_file("dovetail.ini") {
                Ok(config) => {
                    let section_name = format!("{}", env);
                    match config.section(Some(&section_name)) {
                        Some(props) => {
                            println!("[{}]", section_name);
                            for (k, v) in props.iter() {
                                println!("{} = {}", k, v);
                            }
                        }
                        None => eprintln!("Environment '{}' not found.", env),
                    }
                }
                Err(e) => eprintln!("Error loading dovetail.ini: {}", e),
            }
        } else {
            // No env arg passed: show full contents
            match fs::read_to_string("dovetail.ini") {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("Error reading dovetail.ini: {}", e),
            }
        }
    }

    // `dovetail list`
    if let Some(_list) = matches.subcommand_matches("list") {
        match Ini::load_from_file("dovetail.ini") {
            Ok(config) => {
                println!("Environments:");
                for section in config.sections().flatten() {
                    println!("{}", section);
                }
            }
            Err(e) => eprintln!("Error loading dovetail.ini: {}", e),
        }
    }
}
