use std::process::Command;
use std::path::Path;
use std::io;
use chrono::Utc;

fn main() {
    // ----- Compile Date -----
    println!("cargo:rustc-env=COMPILE_DATE={}", Utc::now().format("%Y-%m-%d"));

    // Check if it's cloned from GitHub or a Mirror
    if Path::new(".git").exists() {
        // Get the current branch name
        match git(&["symbolic-ref", "--short", "HEAD"]) {
            Ok(branch) => println!("cargo:rustc-env=GIT_BRANCH_TAG={}", branch),
            Err(_) => {
                // Try to get tag if you're not on a branch
                match git(&["describe", "--tags", "--exact-match"]) {
                    Ok(tag) => println!("cargo:rustc-env=GIT_BRANCH_TAG={}", tag),
                    Err(_) => {}
                }
            }
        }
        // Get Latest Commit Hash
        match git(&["rev-parse", "--short", "HEAD"]) {
            Err(_) => {},
            Ok(hash) => println!("cargo:rustc-env=GIT_HASH={}", hash),
        };
    }
}

fn git(args: &[&str]) -> io::Result<String> {
    let output = Command::new("git")
    .args(args)
    .output()?;
    Ok(String::from_utf8(output.stdout).unwrap())
}