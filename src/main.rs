mod utility;

use std::collections::HashMap;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("No command specified!");
        println!("Usage: flux [command] [...args]");
        println!("Commands: run");
        std::process::exit(1);
    }

    if args[1] == "--help" {
        println!("Usage: flux [command] [...args]");
        println!("Commands: run, interpret");
        std::process::exit(0);
    }

    match args[1].as_str() {
        "run" => run_handler(args),
        _ => println!("Unknown command: {}! Use --help for help!", args[1]),
    }

    Ok(())
}

fn run_handler(args: Vec<String>) {
    let current_dir = std::env::current_dir().expect("Could not get current directory!");
    let flux_cfg_path: &str = "flux.srb";
    let full_flux_cfg_path = current_dir.join(flux_cfg_path);

    if !full_flux_cfg_path.exists() {
        println!("Could not find file {}!", flux_cfg_path);
        std::process::exit(1);
    }

    let contents = match fs::read_to_string(full_flux_cfg_path) {
        Ok(content) => content,
        Err(error) => panic!("Error reading file: {}", error),
    };

    let data: HashMap<String, String> = sorbet::parse(contents);

    if args.len() < 3 {
        let available_tasks = data
            .keys()
            .map(|key| key.trim_matches(&['[', ']', '"'][..]))
            .collect::<Vec<_>>();

        println!("No task specified!");
        println!("Available tasks: {}", available_tasks.join(", "));
        std::process::exit(1);
    }

    let task = args[2].to_string();
    let command = match data.get(&task) {
        Some(cmd) => cmd,
        None => {
            println!("Task '{}' not found!", task);
            println!(
                "Available tasks: {}",
                data.keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            std::process::exit(1);
        }
    };

    utility::flux_print(format!("Running task: {}", task).as_str());
    utility::run_cmd(command, &args[3..]).expect("Failed to run command!");
}

