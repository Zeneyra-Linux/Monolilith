use std::io;
use std::fs;
use std::env::current_dir;
use kagero::printer::{Printer, Colors};
use crate::config::{config, Project};

/// Builds the Project
/// 
/// Builds the Project listed in the `monolilith.json` file.
/// Returns an [Error](io::Error) if it can't read or parse the config.
pub fn build(verbose: bool) -> Result<u128, io::Error> {
    let cwd = current_dir()?;

    // Create build output folder
    if let Err(err) = fs::create_dir(cwd.join("build/")) {
        match err.kind() {
            // Do nothing if the directory already exists
            io::ErrorKind::AlreadyExists => {},
            _ => return Err(err),
        }
    }

    // Read config, make it iterable and filter out invalid Project
    let projects = config()?.into_iter().filter_map(|x| {
        if let Some(project) = Project::from_str(x.1.as_str()) {
            return Some((x.0, project));
        }
        None
    });

    // Printer
    let mut printer = Printer::default();
    // Count for failed Project
    let mut failed = 0;

    // Build each project
    for entry in projects {
        // Info Message
        printer.print("Compiling ", Colors::Cyan)
        .print(&entry.0, Colors::CyanBright)
        .write(" (")
        .print(entry.1.rich(), Colors::CyanBright)
        .println(")...", Colors::Cyan);

        if let Err(err) = entry.1.build(&entry.0, cwd, verbose) {
            // TODO: Handle returned Errors here and print info message
        } else {
            // TODO: Print success info message
        }
    }

    Ok(failed)
}