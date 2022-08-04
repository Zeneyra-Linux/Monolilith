use kagero::printer::{Colors, Printer};
use std::env;
use std::process::exit;

mod config;
mod compile;
mod tasks;

fn main() {
    let mut prnt = Printer::default();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args.get(1).unwrap().as_str() {
            "build" => {
                // Build the project
            },
            "init" => {
                // Initialize the monolilith.json file
            },
            "add" => {
                // Adds a new project
            },
            "help" => {
                // Print help for the app or a specific command
            },
            "version" => {
                // Print the version of the app
            },
            "info" => {
                // Print info about the app
            },
            cmd => {
                prnt.error("Unknown command: ", Colors::Red);
                prnt.errorln(cmd, Colors::RedBright);

                prnt.print("See ", Colors::Yellow);
                prnt.print("monolilith help", Colors::YellowBright);
                prnt.println(" for more!", Colors::Yellow);
                
                exit(1);
            }
        }
    } else {
        // Build the project
    }
    exit(0);
}