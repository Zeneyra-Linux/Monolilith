use std::ops::Add;
use crate::meta;
use kagero::printer::{Printer, Colors};

/// Entry
/// 
/// Prints an entry with color only for the key
fn entry(p: &mut Printer, c: Colors, k: &str, v: &str) {
    entry_color(p, c, k, Colors::None, v)
}

/// Entry with color
/// 
/// Prints an entry with color for both the key and value.
fn entry_color(p: &mut Printer, ck: Colors, k: &str, cv: Colors, v: &str) {
    // Key
    p.print(String::from(k).add(":").as_str(), ck).write(" ")
    // Value
    .println(v, cv);
}

/// Help
/// 
/// Prints Monolilith's help message
pub fn help(p: &mut Printer) {
    // Usage
    entry(p, Colors::RedBright, "USAGE", format!("{} [COMMAND/FLAGS] [ARGS/FLAGS]", meta::NAME).as_str());
    p.println("If no command is set, it will run the build command.", Colors::Red);
    p.writeln("");

    // Commands
    p.println("COMMANDS:", Colors::CyanBright);
    entry(p, Colors::Cyan, "help", "Displays this help message.");
    entry(p, Colors::Cyan, "info", "Displays rich info about the executable.");
    entry(p, Colors::Cyan, "version", "Displays a one-line version message.");
    entry(p, Colors::Cyan, "init", "Creates an empty monolilith.json file in the current directory.");
    entry(p, Colors::Cyan, "build", "Builds all projects listed inside the monolilith.json file.");
    entry(p, Colors::Cyan, "remove <ProjectPath>", "Removes a project to the monolilith.json file.");
    entry(p, Colors::Cyan, "add <ProjectPath> <ProjectType>", "Adds a new project to the monolilith.json file.");
    p.writeln("");

    // Args
    p.println("ARGS:", Colors::CyanBright);
    entry(p, Colors::Cyan, "ProjectPath", "A path relative to the root of the repository. The subfolder name has to match the resulting binary name.");
    entry(p, Colors::Cyan, "ProjectType", format!("The project type. See {}/wiki/Project-Types.", meta::REPOSITORY).as_str());
    p.writeln("");

    // Flags
    p.println("FLAGS:", Colors::GreenBright);
    entry(p, Colors::Green, "-h, --help", "Alias for Command help.");
    entry(p, Colors::Green, "-V, --version", "Alias for Command version.");
    entry(p, Colors::Green, "-v, --verbose", "Option for Command build. Prints the shell output of the executed build commands.")
}