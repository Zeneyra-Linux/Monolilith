use crate::config::config;
use std::io;
use std::fs;
use serde_json::to_string_pretty;

/// Remove Project
/// 
/// Removes a project from the `monolilith.json` file.
pub fn remove(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() < 3 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Missing Project Name!"));
    }

    let mut database = config()?;
    let project = args.get(2).unwrap().to_string();

    if let None = database.remove(&project) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Project {} not found!", project)
        ));
    }

    let data = to_string_pretty(&database).unwrap();
    fs::write("monolilith.json", data)?;
    Ok(())
}