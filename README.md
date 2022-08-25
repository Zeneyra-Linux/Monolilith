# Monolilith
Simple Build System for Monorepos

# Installing
## Cargo
```sh
cargo install monolilith
```

## Kagero
WIP.

## Source
1. Clone the repository: `git clone https://github.com/Zeneyra-Linux/Monolilith.git`
2. Go into the directory: `cd Monolilith`
3. Install or build the application: `cargo install --path .` or `cargo build --release`

Optionally, you can add `RUSTFLAGS="-C target-cpu=native"` in front of the command to optimize the application for your CPU. Same goes for [installing with Cargo](#cargo)

# Usage
## Project structure
You'll need a `monolilith.json` file. It contains a Map where the Keys are the relative paths to the application folder and the Value the [Project Type](https://github.com/Zeneyra-Linux/Monolilith/wiki/Project-Types).  
Example:
```json
{
    "true": "zigcc",
    "false": "zigc++",
    "fetch": "zig",
    "my-app": "cargo-zigbuild",
    "other-apps/server-app": "cargo",
    "other-apps/client-app": "clang",
    "gnuapp": "gcc"
}
```
You can use `monolilith init` to create an empty file. The subproject folder should also match the project name and resulting binary.

## Adding and removing
Use `monolilith add <PathToProject> <ProjectType>` to add a new project or `monolilith remove <PathToProject>`.  
Note that PathToProject must be formatted like [above](#project-structure).

## Building
Just run `monolilith` and it will build all the projects and put the result into `build/`.

# License
Monolilith is currently licensed under the EUPL-1.2 with German law, since I live in Germany, but it will soon be relicensed under the [Zeneyra Public License](https://github.com/Zeneyra-Linux/Monolilith) Version 1.0 with default conditions.