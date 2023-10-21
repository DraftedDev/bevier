use include_dir::{include_dir, Dir};
use std::process::Command;

pub static SIMPLE_3D: Dir = include_dir!("assets/simple_3d");
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
