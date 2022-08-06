const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const NAME_RICH: &str = "Monolilith";
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const LICENSE: &str = "ZPL-1.0";
const LICENSE_RICH: &str = "Zeneyra Pulic License 1.0";

use kagero::printer::{Printer, Colors};
use std::env::consts::{ARCH, OS};

pub fn info(printer: &mut Printer) {
    // TODO: Print info about the app
    printer.println(format!("{} - {}", NAME_RICH, DESCRIPTION).as_str(), Colors::CyanBright);
    // Version
    printer.print("Version: ", Colors::Cyan);
    printer.writeln(VERSION);
    // Repository
    printer.print("Repository: ", Colors::Cyan);
    printer.writeln(REPOSITORY);
    // License
    printer.print("License: ", Colors::Cyan);
    printer.write(LICENSE_RICH);
    printer.write(" (");
    printer.write(LICENSE);
    printer.writeln(")");
    // OS
    printer.print("OS: ", Colors::Cyan);
    printer.writeln(OS);
    // Arch
    printer.print("Arch: ", Colors::Cyan);
    printer.writeln(ARCH);
}

pub fn version(printer: &mut Printer) {
    printer.write(NAME);
    printer.write(" ");
    printer.write(VERSION);
    printer.write(" ");
    printer.write(OS);
    printer.write("/");
    printer.writeln(ARCH);

    // TODO: Add Git Commit Hash and Date from build.rs
}