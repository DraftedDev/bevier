use std::env::current_dir;
use std::fs;

use crate::utils::{clear, SIMPLE_3D};
use inquire::error::InquireResult;
use inquire::{Confirm, Select, Text};
use log::info;

pub fn init() -> InquireResult<()> {
    let name = Text::new("Project Name").prompt()?;

    let template =
        Select::new("Project Type", vec!["Empty App", "Simple 2D", "Simple 3D"]).prompt()?;

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

        fs::create_dir(&project_path).expect("Could not create directory");

        match template {
            "Empty App" => todo!(),
            "Simple 2D" => todo!(),
            "Simple 3D" => {
                SIMPLE_3D.extract(project_path)?;
            }
            _ => panic!("Unknown template!"),
        }
    } else {
        info!("Cancelled! Exiting...");
    }

    Ok(())
}
