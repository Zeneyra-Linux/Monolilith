const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const NAME_RICH: &str = "Monolilith";
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const LICENSE: &str = "EUPL-1.2";
const LICENSE_RICH: &str = "European Union Pulic License 1.2";

// Build Script Metadata
const CARGO_VERSION: &str = env!("CARGO_VERSION");
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
const COMPILE_DATE: &str = env!("COMPILE_DATE");
static GIT_BRANCH_TAG: Option<&str> = option_env!("GIT_BRANCH_TAG");
static GIT_HASH: Option<&str> = option_env!("GIT_HASH");

use kagero::printer::{Printer, Colors};
use std::env::consts::{ARCH, OS};

pub fn info(printer: &mut Printer) {
    // Name and Description
    printer.println(format!("{} - {}", NAME_RICH, DESCRIPTION).as_str(), Colors::CyanBright);

    // Repository
    printer.print("Repository: ", Colors::Cyan);
    printer.writeln(REPOSITORY);

    // License
    printer.print("License: ", Colors::Cyan);
    printer.write(LICENSE_RICH);
    printer.write(" (");
    printer.write(LICENSE);
    printer.writeln(")");

    // Version
    printer.print("Version: ", Colors::Cyan);
    printer.writeln(VERSION);

    // Arch
    printer.print("Arch: ", Colors::Cyan);
    printer.writeln(ARCH);

    // OS
    printer.print("OS: ", Colors::Cyan);
    printer.writeln(OS);

    // Git Branch/Tag and Hash
    printer.print("Build: ", Colors::Cyan);
    if GIT_HASH.is_some() && GIT_BRANCH_TAG.is_some() {
        printer.write(GIT_BRANCH_TAG.unwrap());
        printer.write("@");
        printer.write(GIT_HASH.unwrap());
        printer.write(" (");
        printer.write(COMPILE_DATE);
        printer.writeln(")");
    } else {
        printer.writeln(COMPILE_DATE);
    }

    // Rust Version
    printer.print("Rust: ", Colors::Cyan);
    printer.writeln(RUSTC_VERSION);

    // Cargo Version
    printer.print("Cargo: ", Colors::Cyan);
    printer.writeln(CARGO_VERSION);
}

pub fn version(printer: &mut Printer) {
    // Basic Info
    printer.write(NAME);
    printer.write(" ");
    printer.write(VERSION);
    printer.write(" ");
    printer.write(OS);
    printer.write("/");
    printer.write(ARCH);

    // Custom Compile Info
    printer.write(" (");
    if GIT_BRANCH_TAG.is_some() {
        printer.write(GIT_BRANCH_TAG.unwrap());
        printer.write(" ");
    }
    if GIT_HASH.is_some() {
        printer.write(GIT_HASH.unwrap());
        printer.write(" ");
    }
    printer.write(COMPILE_DATE);
    printer.writeln(")");
}