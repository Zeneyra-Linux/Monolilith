use std::fs;
use std::io;
use crate::config::{parse_config, Projects};
use kagero::runner::{result, shell};
use kagero::printer::{Printer, Colors};

/// Builds the projects
/// 
/// Builds the projects listed in the `monolilith.json` file.
/// Returns an [Error](io::Error) if it can't read the contig or parse the config.
pub fn build() -> Result<u128, io::Error>{
    // Read and Parse config
    let data = fs::read_to_string("monolilith.json")?;
    let monolilith = parse_config(data.as_str())?;

    // Build each project
    let mut failed = 0;
    for (path, projecttype) in monolilith.into_iter() {
        // TODO: Add switch case for project types and general logic to run build commands in module compile
    }
    Ok(failed)
}