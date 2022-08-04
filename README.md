# Monolilith
Simple build system for Monorepos

# Installing
WIP.

# Usage
## Project structure
You'll need a `.monolilith.json` file. It contains a Map where the Keys are the relative paths to the application folder and the Value the [Project Type](https://github.com/Zeneyra-Linux/Monolilith/wiki/Project-Types).  
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

## Building
Just run `monolilith` and it will build all the projects and put the result into `build/`.