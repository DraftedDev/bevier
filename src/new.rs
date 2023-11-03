use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::utils::{
    clear, CARGO_TOML_PLACEHOLDER, DEFAULT_CONFIG, EMPTY_APP, PERFORMANCE_CONFIG, SIMPLE_3D,
    SIZE_CONFIG,
};
use inquire::error::InquireResult;
use inquire::{Confirm, Select, Text};
use log::info;

pub fn init() -> InquireResult<()> {
    let name = Text::new("Project Name").prompt()?;

    let template = Select::new("Project Type", vec!["Empty App", "Simple 3D"]).prompt()?;

    let config = Select::new("Project Config", vec!["Default", "Size", "Performance"]).prompt()?;

    clear();

    info!("Project Name: {}", name);
    info!("Project Type: {}", template);
    info!("Project Config: {}", config);

    if Confirm::new("Do you want to continue?").prompt()? {
        clear();
        info!("Generating...");

        let project_path = current_dir()
            .expect("Could not get current directory")
            .join(name);

        // generate project
        fs::create_dir(project_path.clone()).expect("Could not create directory");

        let cargo_config = match config {
            "Default" => DEFAULT_CONFIG,
            "Size" => SIZE_CONFIG,
            "Performance" => PERFORMANCE_CONFIG,
            _ => panic!("Unknown config!"),
        };

        let cargo_config_path = project_path.clone().join(".cargo");

        fs::create_dir(cargo_config_path.clone()).expect("Could not create directory");

        File::create(cargo_config_path.join("config.toml"))
            .expect("Could not create file")
            .write_all(cargo_config)
            .expect("Could not write file");

        match template {
            "Empty App" => {
                EMPTY_APP.extract(project_path.clone())?;
            }
            "Simple 3D" => {
                SIMPLE_3D.extract(project_path.clone())?;
            }
            _ => panic!("Unknown template!"),
        }

        fs::rename(
            project_path.clone().join(CARGO_TOML_PLACEHOLDER),
            project_path.clone().join("Cargo.toml"),
        )?;
    } else {
        info!("Cancelled! Exiting...");
    }

    Ok(())
}
