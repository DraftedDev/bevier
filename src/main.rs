use env_logger::WriteStyle;
use inquire::error::InquireResult;
use inquire::Select;
use log::{info, LevelFilter};

pub mod new;
pub mod utils;

fn main() -> InquireResult<()> {
    #[cfg(not(debug_assertions))]
    let filter = LevelFilter::Info;
    #[cfg(debug_assertions)]
    let filter = LevelFilter::Debug;

    env_logger::builder()
        .filter_level(filter)
        .format_timestamp(None)
        .format_module_path(false)
        .write_style(WriteStyle::Always)
        .parse_default_env()
        .init();

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
