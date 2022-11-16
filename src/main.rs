pub mod create;

use std::any::{Any, TypeId};
use clap::{Arg, ArgAction, Command};
use std::string::String;
use clap::error::{DefaultFormatter, Error, ErrorKind};
use crate::create::{Config, create, Settings};

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
                .short_flag('c')
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
                        .help("Set the configuration of the project to generate.")
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        ).get_matches();

    match matches.subcommand() {
        // match commands

        Some(("create", args)) => {

            let config = args.get_one::<String>("config");

            let mut settings = Settings::default();

            settings.name = args.get_one::<String>("name").unwrap().to_string();

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

                    _ => {
                        Error::<DefaultFormatter>::raw(ErrorKind::InvalidValue, "Invalid config! Valid configs: 'size', 'performance', 'buildSpeed', 'none'").exit();
                    }
                }
            }

            // if everything is valid
            create(&settings);
        }
        _ => unreachable!(),
    }
}