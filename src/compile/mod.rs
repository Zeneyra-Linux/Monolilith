use std::path::Path;
use std::fs;
use std::io;

/// Cargo
/// 
/// Module for Cargo-specific commands
pub mod cargo;

/// Zig
/// 
/// Module for Zig-specific commands
pub mod zig;

/// GCC
/// 
/// Module for GCC-specific commands
pub mod gcc;

/// Clang
/// 
/// Module for Clang-specific commands
pub mod clang;

/// Go
/// 
/// Module for Go-specific commands
pub mod go;


/// List Directory Entries
/// 
/// List Directory Entries as [Metadata](fs::Metadata) in an [Iterator]
pub fn list_meta(dir: impl AsRef<Path>) -> io::Result<impl Iterator<Item = (fs::DirEntry, fs::Metadata)>> {
    // Read Directory and Iterate over Files
    let entries = fs::read_dir(dir)?.into_iter()
    // Filter Out unavailable entries
    .filter_map(|entry| {
        // Check if entry is available
        if let Ok(direntry) = entry {
            // Check if entry's metadata is available
            if let Ok(filemeta) = direntry.metadata() {
                return Some((direntry, filemeta));
            }
        }
        None
    });
    Ok(entries)
}


/// List Files by Extension
/// 
/// Lists all files in a directory by extension.
/// Used for things like C and Go compiling where all root files have to be set.
/// 
/// `ext` should be the file extension WITHOUT the dot, e.g. `rs`, `go`, `c`, etc.
/// 
/// # Example
/// ```
/// fn main() {
///     for ele in compile::list_files("./src", "rs").unwrap() {
///         println!("{}", ele);
///     }
/// }
/// ``` 
pub fn list_files(dir: impl AsRef<Path>, ext: &str) -> io::Result<Vec<String>> {
    let files_iter = list_meta(dir)?
    // Filter out non-files
    .filter(|x| x.1.is_file())
    // Filter by extension
    .filter(|y| {
        if let Some(extension) = y.0.path().extension() {
            if let Ok(extend) = extension.to_owned().into_string() {
                return extend.ends_with(ext);
            }
        }
        false
    });

    // Cast iter to Vec<String> containing the file names
    Ok(files_iter.filter_map(|x| {
        // Filter by successful conversion from OsString to String
        if let Ok(filename) = x.0.file_name().into_string() {
            return Some(filename);
        }
        None
    }).collect())
}