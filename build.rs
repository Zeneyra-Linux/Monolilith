use std::process::Command;
use std::path::Path;
use std::io;
use std::env;
use chrono::Utc;

fn main() {
    // ----- Target Triple -----
    println!("cargo:rustc-env=TARGET_ENV={}", env::var("CARGO_CFG_TARGET_ENV").unwrap());

    // ----- Rust Version -----
    println!("cargo:rustc-env=CARGO_VERSION={}", version("cargo"));
    println!("cargo:rustc-env=RUSTC_VERSION={}", version("rustc"));

    // ----- Compile Date -----
    println!("cargo:rustc-env=COMPILE_DATE={}", Utc::now().format("%Y-%m-%d"));

    // ----- Git Data -----
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

fn version(name: &str) -> String {
    let output = Command::new(name).arg("--version").output().unwrap();
    let raw = String::from_utf8(output.stdout).unwrap();
    raw.replace(&format!("{} ", name).to_string(), "")
}