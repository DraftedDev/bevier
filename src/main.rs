use inquire::error::InquireResult;
use inquire::Select;
use log::info;

pub mod logging;
pub mod new;
pub mod utils;

fn main() -> InquireResult<()> {
    logging::init_console();

    info!(
        "Welcome to bevier v{} // Made by https://DraftedDev.github.io",
        env!("CARGO_PKG_VERSION")
    );

    match Select::new("What should I do?", vec!["New Project", "Quit"]).prompt()? {
        "New Project" => new::init()?,
        "Quit" => info!("Bye!"),
        _ => panic!("Failed to select option"),
    }

    Ok(())
}
