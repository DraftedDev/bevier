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
}

pub fn create(settings: &Settings) {

    // create cargo project
    Command::new("cargo")
        .args(&["-q", "new", "--bin", settings.name.as_str(), ])
        .spawn()
        .expect("Failed to execute cargo-new command!");

    println!("Created Cargo Project!");

    // I'm too lazy for implementing awaiting the cargo command, so just sleep a while
    // TODO: Implement awaiting for last command to finish
    thread::sleep(Duration::new(2, 20));

    let mut project_dir = current_dir().unwrap();
    project_dir.push(&settings.name);

    // cd to project dir
    set_current_dir(Path::new(project_dir.as_path())).expect("Could not set current dir to Project dir!");

    // edit main.rs
    fs::write("src/main.rs", "\
use bevy::prelude::*;\n
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
    ")
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

    /*
    Configure project for different targets.
    TODO: make cargo automatically add bevy/dynamic feature on debug
    TODO: add smart module (for pro-devs)
    TODO: benchmark modules
     */
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

        /*
        Config::Smart => {
            // size config for web builds
            // performance config for native builds
            // faster debug builds
        }
         */
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