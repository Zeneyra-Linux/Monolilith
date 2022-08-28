pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const NAME_RICH: &str = "Monolilith";
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const LICENSE: &str = "EUPL-1.2";
pub const LICENSE_RICH: &str = "European Union Public License 1.2";

// Build Script Metadata
pub const CARGO_VERSION: &str = env!("CARGO_VERSION");
pub const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub const COMPILE_DATE: &str = env!("COMPILE_DATE");
pub static GIT_BRANCH_TAG: Option<&str> = option_env!("GIT_BRANCH_TAG");
pub static GIT_HASH: Option<&str> = option_env!("GIT_HASH");

use kagero::printer::{Printer, Colors};
use std::env::consts::{ARCH, OS};

pub fn info(printer: &mut Printer) {
    // Name and Description
    printer.println(format!("{} - {}", NAME_RICH, DESCRIPTION).as_str(), Colors::CyanBright);

    // Repository
    printer.print("Repository: ", Colors::Cyan).writeln(REPOSITORY);

    // License
    printer.print("License: ", Colors::Cyan).write(LICENSE_RICH)
    .write(" (").write(LICENSE).writeln(")");

    // Version
    printer.print("Version: ", Colors::Cyan).writeln(VERSION);

    // Arch
    printer.print("Arch: ", Colors::Cyan).writeln(ARCH);

    // OS
    printer.print("OS: ", Colors::Cyan).writeln(OS);

    // Git Branch/Tag and Hash
    printer.print("Build: ", Colors::Cyan);
    if GIT_HASH.is_some() && GIT_BRANCH_TAG.is_some() {
        printer.write(GIT_BRANCH_TAG.unwrap())
        .write("@").write(GIT_HASH.unwrap())
        .write(" (").write(COMPILE_DATE).writeln(")");
    } else {
        printer.writeln(COMPILE_DATE);
    }

    // Rust Version
    printer.print("Rust: ", Colors::Cyan).writeln(RUSTC_VERSION);

    // Cargo Version
    printer.print("Cargo: ", Colors::Cyan).writeln(CARGO_VERSION);
}

pub fn version(printer: &mut Printer) {
    // Basic Info
    printer.write(NAME).write(" ")
    .write(VERSION).write(" ")
    .write(OS).write("/").write(ARCH);

    // Custom Compile Info
    printer.write(" (");
    if GIT_BRANCH_TAG.is_some() {
        printer.write(GIT_BRANCH_TAG.unwrap()).write(" ");
    }
    if GIT_HASH.is_some() {
        printer.write(GIT_HASH.unwrap()).write(" ");
    }
    printer.write(COMPILE_DATE).writeln(")");
}
