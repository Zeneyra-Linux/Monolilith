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
                tasks::build();
            },
            "init" => {
                match tasks::init() {
                    Ok(msg) => prnt.println(msg.as_str(), Colors::Green),
                    Err(_) => {
                        prnt.errorln("Could not create monolilith.json", Colors::Red);
                        exit(1);
                    }
                }
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
        tasks::build();
    }
    exit(0);
}