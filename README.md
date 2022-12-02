# A CLI-Tool to generate Bevy Engine Games

## Getting Started
- Install with `cargo install bevier`
- Create your project executing `bevier create --name "my_bevy_game"`

## Full Usage
````text
create, -c  Create a new Bevy Engine Project.
help        Print this message or the help of the given subcommand(s)

Options:
-h, --help  Print help information
````
***

### ``create`` Command
````text
Usage: bevier {create|-c} [OPTIONS] --name <name>

Options:
  -n, --name <name>         The name of the Project.
  -o, --configure <config>  Set the configuration of the project to generate.
  -h, --help                Print help information
````

## Configurations
- none: Only adds some options for compatibility
- performance: For optimal performance on debug and release builds
- size: Optimizes the size of the release build
- buildSpeed: Use if you don't want to wait that long for debug builds
- smart: A mix of every config
- potato: Made for potato PCs that can't run stuff that fast

## TODO
- Add benchmarking module
- Add --example option to copy a bevy example project
- Make bevy automatically use the bevy/dynamic feature when debug mode