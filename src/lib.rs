use crate::config::Config;
use std::process::Command;
pub mod colors;
pub mod config;

/// Easy wrapper around Rust's `Command::new` utility
pub fn spawn(command: &str) {
    Command::new("riverctl")
        .args(["spawn", command])
        .spawn()
        .expect("Can't launch program");
}

/// This is just an example of using the library
fn main() {
    let mut config: Config = Config::new();
    let keybinds = vec![
        ["Q", "spawn ghostty"],
        ["C", "close"],
        ["J", "focus-view next"],
        ["K", "focus-view previous"],
        ["M", "spawn wlogout"],
        ["B", "spawn zen-browser"],
    ];
    let shift_keybinds = vec![["E", "exit"], ["J", "swap next"], ["K", "swap previous"]];
    config
        .set_keybinds(keybinds)
        .change_super("Super+Shift")
        .set_keybinds(shift_keybinds);
    print!("{:?}", config);
}
