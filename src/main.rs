pub mod create;

use clap::{Arg, ArgAction, Command};
use std::string::String;
use clap::error::{DefaultFormatter, Error, ErrorKind};
use crate::create::{Config, create, Example, Settings};

fn main() {

    // the command
    let matches = Command::new("bevier")
        .about("[ A CLI for creating games with the Bevy Engine ]")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("DraftedDev")
        // 'Create' Command
        .subcommand(
            Command::new("create")
                .about("Create a new Bevy Engine Project.")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .help("The name of the Project.")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("config")
                        .long("configure")
                        .short('o')
                        .help(r#"
Set the configuration of the project to generate.
Configuration Help:
    - "none" No config, only set some values for compatiblity (default)
    - "performance" Configure for performance
    - "buildSpeed" Configure for development and release build speed
    - "size" Configure for size
    - "smart" Basically every config mixed with some other settings
    - "potato" If you develop with a trashy PC, then choose this config
"#)
                        .action(ArgAction::Set)
                        .num_args(1),
                )
                .arg(
                    Arg::new("example")
                        .long("example")
                        .short('x')
                        .help(r#"
Generate Project as a premade example.
Example Help:
    - "3d" A basic 3d Scene containing some Meshes and lighting
    - "2d" A basic 2d Scene containing a single Sprite
    - "breakout" The breakout game from the bevy website
    - "ui" A basic App with some UI Elements
    - "none" Just an empty Bevy App (default)
"#)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        ).get_matches();

    match matches.subcommand() {
        // match commands

        Some(("create", args)) => {

            let config = args.get_one::<String>("config");
            let example = args.get_one::<String>("example");

            let mut settings = Settings::default();

            settings.name = args.get_one::<String>("name").unwrap().to_string();

            // Set the Project Template/Example
            if example.is_some() {
                match example.unwrap().to_string().as_str() {

                    "ui" => {
                        settings.example = Example::Ui;
                    }

                    "3d" => {
                        settings.example = Example::Scene3D;
                    }

                    "2d" => {
                        settings.example = Example::Scene2D;
                    }

                    "breakout" => {
                        settings.example = Example::GameBreakout;
                    }

                    "none" => {
                        settings.example = Example::None;
                    }

                    _ => {
                        Error::<DefaultFormatter>::raw(ErrorKind::InvalidValue, r#"
Invalid example! Valid examples: '3d', '2d', 'ui', 'breakout', 'none'.
For example help, do "bevier create --help".
                        "#).exit();
                    }
                }
            }

            // Configure Project
            if config.is_some() {
                match config.unwrap().to_string().as_str() {

                    "performance" => {
                        settings.config = Config::Performance;
                    }

                    "none" => {
                        settings.config = Config::None;
                    }

                    "buildSpeed" => {
                        settings.config = Config::BuildSpeed;
                    }

                    "size" => {
                        settings.config = Config::Size;
                    }

                    "smart" => {
                        settings.config = Config::Smart;
                    }

                    "potato" => {
                        settings.config = Config::Potato;
                    }

                    _ => {
                        Error::<DefaultFormatter>::raw(ErrorKind::InvalidValue, r#"
Invalid config! Valid configs: 'size', 'performance', 'buildSpeed', 'potato', 'none'.
For configuration help, do "bevier create --help".
                        "#).exit();
                    }
                }
            }

            // if everything is valid
            create(&settings);
        }
        _ => unreachable!(),
    }
}