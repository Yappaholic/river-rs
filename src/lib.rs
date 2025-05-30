//! # River-rs
//!
//! ## WIP
//!
//! This is a simple library, providing Rust bindings to create and use
//! RiverWM config, instead of writing everything in the shell file.
//!
use std::process::Command;
pub mod colors;
pub mod config;
pub mod layout;

/// Easy wrapper around Rust's `Command::new` utility
pub fn spawn(command: &str) {
    Command::new("riverctl")
        .args(["spawn", command])
        .spawn()
        .expect("Can't launch program")
        .wait()
        .unwrap();
}

#[cfg(test)]
mod test {
    use crate::config::Config;
    use crate::config::KeyboardLayout;
    // Panics without RiverWM installed
    // Returns riverctl error when RiverWM installed
    #[test]
    fn create_config() {
        let mut config: Config = Config::default();
        let autostart = vec!["waybar -c ~/.config/waybar/riverconfig"];
        let layout: KeyboardLayout<&str> = KeyboardLayout {
            rules: None,
            model: None,
            variant: Some("colemak_dh_wide_iso,"),
            options: Some("grp:toggle,ctrl:nocaps"),
            layout: Some("us,ru"),
        };
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
            .set_mouse_keybinds(Some("move-view"), Some("resize-view"), None)
            .change_super("Super+Shift")
            .set_keybinds(shift_keybinds)
            .set_tags("Super", "Super+Shift")
            .set_keyboard_layout(layout)
            .autostart(autostart);
        config.print_keybindings();
    }
}
