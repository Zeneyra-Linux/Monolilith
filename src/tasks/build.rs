use std::io;
use crate::config::{config, Project};

/// Builds the Project
/// 
/// Builds the Project listed in the `monolilith.json` file.
/// Returns an [Error](io::Error) if it can't read or parse the config.
pub fn build(verbose: bool) -> Result<u128, io::Error>{
    // Count for failed Project
    let mut failed = 0;

    // Read config, make it iterable and filter out invalid Project
    config()?.into_iter().filter_map(|x| {
        if let Some(project) = Project::from_str(x.1.as_str()) {
            return Some((x.0, project));
        }
        None
    })
    // Build each project
    .for_each(|entry| entry.1.build(&entry.0, verbose));

    Ok(failed)
}