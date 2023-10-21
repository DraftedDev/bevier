use env_logger::WriteStyle;
use log::LevelFilter;

pub fn init_console() {
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
}