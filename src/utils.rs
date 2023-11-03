use include_dir::{include_dir, Dir};
use std::process::Command;

pub static SIMPLE_3D: Dir = include_dir!("./assets/simple_3d");
pub static EMPTY_APP: Dir = include_dir!("./assets/empty_app");

pub static DEFAULT_CONFIG: &[u8] = include_bytes!("../assets/configs/default.toml");
pub static SIZE_CONFIG: &[u8] = include_bytes!("../assets/configs/size.toml");
pub static PERFORMANCE_CONFIG: &[u8] = include_bytes!("../assets/configs/performance.toml");

pub static CARGO_TOML_PLACEHOLDER: &str = "_cargo_.toml";

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
