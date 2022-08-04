# Monolilith
Simple build system for Monorepos

# Installing
WIP.

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
You can use `monolilith init` to create an empty file.

## Adding and removing
Use `monolilith add <PathToProject> <ProjectType>` to add a new project or `monolilith remove <PathToProject>`.  
Note that `PathToProject` must be formated like above.

## Building
Just run `monolilith` and it will build all the projects and put the result into `build/`.
