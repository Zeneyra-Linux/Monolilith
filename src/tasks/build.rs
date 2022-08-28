use std::io::{Error, ErrorKind};
use std::fs;
use std::env::current_dir;
use kagero::printer::{Printer, Colors};
use crate::config::{config, Project};

/// Builds the Project
/// 
/// Builds the Project listed in the `monolilith.json` file.
/// Returns an [Error](Error) if it can't read or parse the config.
pub fn build(verbose: bool) -> Result<u128, Error> {
    // Read config, make it iterable and filter out invalid Project
    let projects = config()?.into_iter().filter_map(|x| {
        if let Some(project) = Project::from_str(x.1.as_str()) {
            return Some((x.0, project));
        }
        None
    });

    // Create build output folder
    if let Err(err) = fs::create_dir(current_dir()?.join("build/")) {
        match err.kind() {
            // Do nothing if the directory already exists
            ErrorKind::AlreadyExists => {},
            _ => return Err(err),
        }
    }

    // Printer
    let mut printer = Printer::default();
    // Count for failed Project
    let mut failed = 0;

    // State that there are no projects if the table is empty
    let table: Vec<(String, Project)> = projects.collect();
    if table.len() < 1 {
        printer.println("monolilith.json is empty!", Colors::BlueBright);
        return Ok(0);
    }

    // Build each project
    for entry in table {
        // Info Message
        printer.print("Compiling ", Colors::Cyan)
        .print(&entry.0, Colors::CyanBright)
        .print(" for ", Colors::Cyan)
        .print(entry.1.rich(), Colors::CyanBright)
        .println("...", Colors::Cyan);

        if let Err(err) = entry.1.build(current_dir()?.join(&entry.0), current_dir()?, verbose) {
            // Increment failed project count   
            failed += 1;

            // Tell user the issue
            printer.print("Could not build ", Colors::Red)
            .print(&entry.0, Colors::RedBright)
            .println("...", Colors::Red);
            
            // Print the cause
            match err.kind() {
                // TODO: Add precise error explainations here in the future
                _ => printer.errorln(err.to_string().as_str(), Colors::Red)
            };
        } else {
            printer.print("Successfully built ", Colors::Green)
            .print(&entry.0, Colors::GreenBright)
            .println("!", Colors::GreenBright);

        }
        // New line
        printer.writeln("");
    }

    Ok(failed)
}