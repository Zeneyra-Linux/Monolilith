use kagero::printer::{Colors, Printer};
use std::env;
use std::process::ExitCode;

mod config;
mod compile;
mod tasks;

fn main() -> ExitCode {
    let mut prnt = Printer::default();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args.get(1).unwrap().as_str() {
            "build" => {
                return build_handle(&mut prnt);
            },
            "init" => {
                match tasks::init() {
                    Ok(msg) => prnt.println(msg.as_str(), Colors::Green),
                    Err(_) => {
                        prnt.errorln("Could not create monolilith.json", Colors::Red);
                        return ExitCode::FAILURE;
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

                return ExitCode::FAILURE;
            }
        }
    } else {
        return build_handle(&mut prnt);
    }
    ExitCode::SUCCESS
}


/// Build Handle
/// 
/// Handles the build command.
fn build_handle(prnt: &mut Printer) -> ExitCode{
    let failed = match tasks::build() {
        Ok(failed) => failed,
        Err(_) => {
            prnt.errorln("Could not read monolilith.json", Colors::RedBright);

            prnt.error("Please run ", Colors::Red);
            prnt.error("monolilith init", Colors::RedBright);
            prnt.errorln(" to initialize a new build if you haven't already.", Colors::Red);
            
            return ExitCode::FAILURE;
        }
    };
    if failed > 0 {
        prnt.errorln(format!("{} projects failed to build!", failed).as_str(), Colors::RedBright);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}