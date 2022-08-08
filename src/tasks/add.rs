use std::fs;
use std::io;
use std::path::Path;
use serde_json::to_string_pretty;
use crate::config::{parse_config, Projects};

/// Adds a new project
/// 
/// Adds a new project to the `monolilith.json` file.
pub fn add(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Missing Path and/or Project Type"));
    }
    let data = match fs::read_to_string("monolilith.json") {
        Ok(data) => data,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Could not find monolilith.json"))
    };
    let mut config = parse_config(data.as_str())?;

    let path = args.get(2).unwrap().to_string();
    let project = Projects::from_str(args.get(3).unwrap().as_str());

    if !Path::new(&path).exists() && !project.valid() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid Project Type"));
    }
    let err = match config.insert(path, project) {
        Some(_) => true,
        None => false
    };
    
    if err {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Project already exists"));
    }
    let data = to_string_pretty(&config).unwrap();
    fs::write("monolilith.json", data)?;
    return Ok(())
}