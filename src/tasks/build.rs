use std::io;
use crate::config::{config, Project};
use kagero::runner::{result, shell};
use kagero::printer::{Printer, Colors};

/// Builds the Project
/// 
/// Builds the Project listed in the `monolilith.json` file.
/// Returns an [Error](io::Error) if it can't read or parse the config.
pub fn build() -> Result<u128, io::Error>{
    // Count for failed Project
    let mut failed = 0;

    // Read config, make it iterable and filter out invalid Project
    config()?.into_iter().filter(|x| Project::valid_str(x.1.as_str()))

    // Build each project
    .for_each(|entry| {
        // TODO: Actually add logic here
    });

    Ok(failed)
}