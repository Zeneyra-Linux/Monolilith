use std::fs;
use std::io;
use std::path::Path;

/// Initialize new project
/// 
/// Creates an empty project in the current directory.
/// Returns an error if it fails to write to the `monolilith.json` file.
/// Returns the success status as a [String].
pub fn init() -> Result<String, io::Error> {
    if Path::new("monolilith.json").exists() {
        return Ok(String::from("monolilith.json already exists!"));
    }
    fs::write("monolilith.json", "{}")?;
    Ok(String::from("Successfully created monolilith.json!"))
}