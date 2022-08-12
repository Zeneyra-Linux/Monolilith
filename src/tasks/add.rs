use std::fs;
use std::io;
use std::path::Path;
use serde_json::to_string_pretty;
use crate::config::{config, Project};

/// Adds a new project
/// 
/// Adds a new project to the `monolilith.json` file.
pub fn add(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Missing Path and/or Project Type"));
    }
    let mut config = config()?;

    let path = args.get(2).unwrap().to_string();
    let project = Project::from_str(args.get(3).unwrap().as_str());

    let project_path = Path::new(&path);
    if !project_path.exists() || !project_path.is_relative() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid Project Path"));
    }
    if !project.valid() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Project Type"));
    }

    if let Some(_) = config.insert(path, project.to_string()) {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Project already exists"));
    }

    let data = to_string_pretty(&config).unwrap();
    fs::write("monolilith.json", data)?;
    Ok(())
}