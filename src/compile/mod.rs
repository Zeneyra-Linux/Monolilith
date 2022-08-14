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


/// List Files by Extension
/// 
/// Lists all files in a directory by extension.
/// Used for things like C and Go compiling where all root files have to be set.
pub fn list_files(dir: impl AsRef<Path>, ext: &str) -> io::Result<()> {
    // Read Directory and Iterate over Files
    let files = fs::read_dir(dir)?.into_iter()

    // Filter Out unavailable entries and non-files
    .filter_map(|entry| {
        // Check if entry is available
        if let Ok(direntry) = entry {
            // Check if entry's metadata is available
            if let Ok(filemeta) = direntry.metadata() {
                // Check if entry is a file
                if filemeta.is_file() {
                    return Some(filemeta);
                }
            }
        }
        None
    });
    Ok(())
}

/// List Files by Extension recursively
/// 
/// Lists all files in a directory and its subdirectories by extension.
pub fn list_files_recursive(dir: impl AsRef<Path>, ext: &str) {

}