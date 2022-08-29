use kagero::printer::{Colors, Printer};
use std::env;
use std::io::{ErrorKind, Error};
use std::process::ExitCode;

mod config;
mod meta;
mod help;
mod tasks;
mod compile;

fn main() -> ExitCode {
    let mut prnt = Printer::default();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args.get(1).unwrap().as_str() {
            "build" => {
                // Special Build Handle
                if let Some("-v" | "--verbose") = args.get(2).and_then(|x| Some(x.as_str())) {
                    return build_handle(&mut prnt, true);
                }
                return build_handle(&mut prnt, false);
            },
            "-v" | "--verbose" => {
                // Verbose
                return build_handle(&mut prnt, true);
            },
            "init" => {
                // Special Init Handle
                if let Ok(msg) = tasks::init() {
                    prnt.println(msg.as_str(), Colors::Green);
                } else {
                    prnt.errorln("Could not create monolilith.json", Colors::Red);
                    return ExitCode::FAILURE;
                }
            },
            "add" => {
                // Common Result Handler
                let result = tasks::add(args);
                return result_handle(&mut prnt, result, "Successfully added project!");
            },
            "remove" => {
                // Common Result Handler
                let res = tasks::remove(args);
                return result_handle(&mut prnt, res, "Successfully removed project!");
            },
            "help" | "-h" | "--help" => {
                // Help
                help::help(&mut prnt);
            },
            "version" | "-V" | "--version" => {
                // Version
                meta::version(&mut prnt);
            },
            "info" => {
                // Info
                meta::info(&mut prnt);
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
        return build_handle(&mut prnt, false);
    }
    ExitCode::SUCCESS
}

/// Common Result Handle
/// 
/// Handles the result for similar talks like adding and removing.
fn result_handle(prnt: &mut Printer, res: Result<(), Error>, success_message: &str) -> ExitCode {
    match res {
        Ok(_) => prnt.println(success_message, Colors::Green),
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => prnt.errorln("Cannot read or write to monolilith.json", Colors::Red),
                _ => prnt.errorln(e.to_string().as_str(), Colors::Red)
            };
            return ExitCode::FAILURE;
        }
    };
    return ExitCode::SUCCESS;
}

/// Build Handle
/// 
/// Handles the build command.
fn build_handle(prnt: &mut Printer, verbose: bool) -> ExitCode {
    let failed = match tasks::build(verbose) {
        Ok(failed) => failed,
        Err(_) => {
            prnt.errorln("Could not read monolilith.json or create the output folder!!", Colors::RedBright);

            prnt.error("Please run ", Colors::Red);
            prnt.error("monolilith init", Colors::RedBright);
            prnt.errorln(" to create a new build file and make sure you have read and write access.", Colors::Red);
            
            return ExitCode::FAILURE;
        }
    };
    if failed > 0 {
        prnt.errorln(format!("{} project(s) failed to build!", failed).as_str(), Colors::YellowBright);
        return ExitCode::FAILURE;
    } else {
        prnt.println("All projects were successfully built!", Colors::MagentaBright);
    }
    ExitCode::SUCCESS
}
