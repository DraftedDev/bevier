use include_dir::{include_dir, Dir};
use std::process::Command;

pub const SIMPLE_3D: Dir = include_dir!("./assets/simple_3d");
pub const SIMPLE_2D: Dir = include_dir!("./assets/simple_2d");
pub const EMPTY_APP: Dir = include_dir!("./assets/empty_app");

pub const DEFAULT_CONFIG: &[u8] = include_bytes!("../assets/configs/default.toml");
pub const SIZE_CONFIG: &[u8] = include_bytes!("../assets/configs/size.toml");
pub const PERFORMANCE_CONFIG: &[u8] = include_bytes!("../assets/configs/performance.toml");

pub const CARGO_TOML_PLACEHOLDER: &str = "_cargo_.toml";
pub const CARGO_FEATURES_PLACEHOLDER: &str = "\"%FEATURES%\"";

#[inline(always)]
pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

#[inline(always)]
pub fn exec(cmd: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", cmd])
            .spawn()
            .expect("command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new(cmd)
            .spawn()
            .expect("command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
