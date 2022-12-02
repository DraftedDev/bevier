use std::env::{current_dir, set_current_dir};
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::{fs, thread};
use std::io::{Write};
use std::time::Duration;

pub struct Settings {
    pub name: String,
    pub config: Config,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "my_bevy_app".to_string(),
            config: Config::None,
        }
    }
}

pub enum Config {
    Size,
    Performance,
    BuildSpeed,
    None,
    Smart,
    Potato,
}

pub fn create(settings: &Settings) {

    // create cargo project
    let mut cmd = Command::new("cargo")
        .args(&["-q", "new", "--bin", settings.name.as_str(), ])
        .spawn()
        .expect("Failed to execute cargo-new command!");

    // this makes sure that the command has finished executing
    cmd.wait().expect("Failed to wait for cargo-new command!");

    println!("Created Cargo Project!");

    // change execution dir to the dir of the generated project
    let mut project_dir = current_dir().unwrap();
    project_dir.push(&settings.name);

    // cd to project dir
    set_current_dir(Path::new(project_dir.as_path())).expect("Could not set current dir to Project dir!");

    // edit main.rs
    fs::write("src/main.rs", r#"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
    "#)
        .expect("Could not write to main.rs!");

    // add dependencies
    Command::new("cargo")
        .args(&["-q", "add", "bevy"])
        .spawn()
        .expect("Failed to execute cargo-add command!");

    println!("Added dependencies!");

    // configure project
    fs::create_dir(Path::new(".cargo")).expect("Could not create .cargo dir!");

    let mut config_file = File::create(".cargo/config.toml").expect("Could not create config.toml!");

    let config = match settings.config {
        Config::Size => {
            println!("You've chosen the Size config. Your builds will be small and decent in performance, best for Web-Builds.");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 3 # optimize dependencies in debug

[profile.dev]
opt-level = 1 # don't optimize your own code in debug that much
incremental = true

[profile.release]
lto = true # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
panic = "abort" # disables unwinding errors, less size, less debug info
opt-level = "s" # optimize for size
strip = "symbols"

[workspace]
resolver = "2" # Important if you are using workspaces
            "#
        }

        Config::Performance => {
            println!("You've chosen the Performance config. Your builds will be fast and small, but will take centuries to compile.");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 3 # optimize dependencies in debug

[profile.dev]
opt-level = 1 # don't optimize your own code in debug that much

[profile.release]
lto = true # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
opt-level = 3 # optimize for size

[workspace]
resolver = "2" # Important if you are using workspaces
            "#
        }

        Config::BuildSpeed => {
            println!("You've chosen the build speed config. Your builds will compile fast, in debug and release mode, but won't be that fast.");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 2 # optimize dependencies in debug (a bit)

[profile.dev]
opt-level = 0 # don't optimize your own code in debug
incremental = true

[profile.release]
lto = "thin" # enables thin lto to optimize release builds fast
incremental = true

[workspace]
resolver = "2" # Important if you are using workspaces
            "#
        }

        Config::None => {
            println!("You've chosen no config. Your builds won't be much optimized or smaller.");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 3 # optimize dependencies in debug

[profile.dev]
opt-level = 1 # don't optimize your own code in debug that much

[workspace]
resolver = "2" # Important if you are using workspaces
            "#
        }


        Config::Smart => {
            println!("You've chosen no config. Your builds won't be much optimized or smaller.");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 3 # optimize dependencies in debug

[profile.dev]
opt-level = 1 # don't optimize your own code in debug that much
incremental = true

[profile.release]
lto = true # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
# panic = "abort" # uncomment this to get a bit smaller executables, however it will harden debugging release builds
opt-level = 3 # optimize for performance
strip = "symbols"

[workspace]
resolver = "2" # Important if you are using workspaces

[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
lto = true # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
# panic = "abort" # uncomment this to get a bit smaller executables, however it will harden debugging release builds (not recommended for Web-Builds)
opt-level = "s" # optimize for size
strip = "symbols"
            "#
        }

        Config::Potato => {
            println!("You've chosen the potato config. Your game will be perfectly FOR BEING DEVELOPED on a potato. Have Fun :D");
            println!("Do 'cargo run --features bevy/dynamic' for debug builds!");

            r#"
[profile.dev.package."*"]
opt-level = 0 # don't optimize anything

[profile.dev]
opt-level = 0 # don't optimize your own code
incremental = true

[profile.release]
lto = "thin" # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
panic = "abort" # remove if you want better debugging support in release builds
opt-level = "s" # optimize for size
strip = "symbols"

[workspace]
resolver = "2" # Important if you are using workspaces

[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
lto = "thin" # enables fat lto to optimize release build
codegen-units = 1 # the less codegen units, the more optimization, but slow compiling
panic = "abort" # remove if you want better debugging support in release builds
opt-level = "s" # optimize for size
strip = "symbols"
            "#
        }

    };

    // write config
    config_file.write(config.as_bytes()).expect("Could not write to config.toml!");

    // parse default output of get_default_toolchain()
    if get_default_toolchain().contains("nightly") {

        File::create("rust-toolchain.toml").expect("Could not create rust-toolchain.toml file!")
            .write(r#"
[toolchain]
channel = "nightly"
        "#.as_bytes()).expect("Could not write to rust-toolchain.toml file!");

    } else {
        println!("WARNING: It seems you do not use the default nightly toolchain. To get maximum performance switch to the nightly channel.");
    }

}

/// Get the default toolchain
pub fn get_default_toolchain() -> String {
    // convert the output of 'rustup show active-toolchain' to a String
    String::from_utf8(
        Command::new("rustup")
                          .args(&["show", "active-toolchain"])
                          .output()
            .unwrap()
            .stdout
    ).unwrap()
}