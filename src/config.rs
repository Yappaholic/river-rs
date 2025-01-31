//! # Config
//!
//! The heart of river-rs library
use crate::colors::Colors;
use crate::layout::Layout;
use std::io::Result;
use std::process::Command;

/// Struct for info of modifier, keymap and associated command.
///
/// You should not write Keybinds by yourself, that's why `set_keybind` and `set_keybinds` are for.
#[derive(Clone, Debug)]
pub struct Keybind {
    modifier: String,
    keymap: String,
    command: String,
}

/// The heart of the River configuration, holding values for default and mouse-related keybinds, colors and modifier
///
/// Each value can only be changed with methods, because it is Rust, baby!
/// For more info, check associated structs and implemented methods.
#[derive(Debug)]
pub struct Config {
    keybinds: Vec<Keybind>,
    colors: Colors,
    layout: Layout,
    modifier: String,
}

impl Config {
    /// Creates empty config with no keybinds.
    ///
    /// The default modifier is `Super`.
    /// To check the default colors visit Colors struct.
    pub fn new() -> Config {
        Config {
            keybinds: vec![],
            colors: Colors::default(),
            layout: Layout::default(),
            modifier: String::from("Super"),
        }
    }

    /// Sets xkb settings related to repeat_rate and repeat_delay.
    ///
    /// The typematic delay indicates the amount of time (typically in milliseconds) a key needs to be pressed and held in order for the repeating process to begin.
    /// After the repeating process has been triggered, the character will be repeated with a certain frequency (usually given in Hz) specified by the typematic rate.
    ///`(Taken from the Arch Wiki)`
    pub fn set_repeat(&self, repeat_rate: u32, repeat_delay: u32) -> &Self {
        Command::new("riverctl")
            .args([
                "set-repeat",
                repeat_rate.to_string().as_str(),
                repeat_delay.to_string().as_str(),
            ])
            .spawn()
            .expect("Can't set xkb settings");

        return self;
    }

    /// Changes the River Modifier key.
    ///
    /// Useful when chaining `set_keybinds` with different modifiers.
    ///
    /// # Example
    /// ```
    /// use river_rs::config::Config;
    ///
    /// let mut config = Config::new();
    ///
    /// let keybinds = vec![
    ///     ["C", "close"],
    ///     ["J", "focus-view next"],
    ///     ["K", "focus-view previous"],
    /// ];
    /// let shift_keybinds = vec![
    ///     ["E", "exit"],
    ///     ["J", "swap next"],
    ///     ["K", "swap previous"],
    /// ];
    /// config
    ///     .set_keybinds(keybinds)
    ///     .change_super("Super+Shift")
    ///     .set_keybinds(shift_keybinds)
    ///     .apply()
    ///     .unwrap();
    /// ```
    pub fn change_super(&mut self, key: &str) -> &mut Self {
        self.modifier = String::from(key);
        return self;
    }

    /// Sets the single keybind
    ///
    /// # Example
    /// ```
    /// use river_rs::config::Config;
    ///
    /// let mut config = Config::new();
    /// let key = String::from("Q");
    /// let command = String::from("spanw foo");
    /// config.set_keybind(&key, &command);
    /// ```
    pub fn set_keybind(&mut self, keys: &String, command: &String) -> &mut Self {
        let keys = keys.clone();
        let command = command.clone();

        let keybind = Keybind {
            modifier: self.modifier.clone(),
            keymap: keys,
            command,
        };

        self.keybinds.push(keybind);
        return self;
    }

    /// Sets tags from 1 to 9 based on passed modifiers
    pub fn set_tags(&self, modifier: &str, switch_modifier: &str) {
        let tags: Vec<u32> = (0..10).collect();
        let tag_ids: Vec<u32> = tags.iter().map(|x| 2_u32.pow(*x)).collect();
        let mut keybinds: Vec<Keybind> = Vec::new();
        let mut idx = 0;
        while idx < tags.len() {
            keybinds.push(Keybind {
                modifier: String::from(modifier),
                keymap: tags[idx].to_string(),
                command: String::from("set-focused-tags ") + tag_ids[idx].to_string().as_str(),
            });
            keybinds.push(Keybind {
                modifier: String::from(switch_modifier),
                keymap: tags[idx].to_string(),
                command: String::from("set-view-tags ") + tag_ids[idx].to_string().as_str(),
            });
            idx += 1;
        }
    }

    fn apply_colors(&mut self) -> &mut Self {
        let background_color = format!("{:#X}", self.colors.background_color);
        let border_color_focused = format!("{:#X}", self.colors.border_color_focused);
        let border_color_unfocused = format!("{:#X}", self.colors.border_color_unfocused);

        let commands = vec![
            ["background-color", background_color.as_str()],
            ["border-color-focused", border_color_focused.as_str()],
            ["border-color-unfocused", border_color_unfocused.as_str()],
        ];

        for command in commands {
            Command::new("riverctl")
                .args(command)
                .spawn()
                .expect("Can't set colors with riverctl\n");
        }

        return self;
    }

    /// Sets keybinds based on the vector of lists with 2 values
    ///
    /// Second command can be written with spaces, no need to define every argument separatly.
    ///  
    /// Takes the ownership of the vector and modifies it to supply `riverctl`
    ///
    /// # Examples
    /// ```
    /// use river_rs::config::Config;
    ///
    /// let mut config = Config::new();
    /// let keybinds = vec![
    ///   ["Q", "spawn foo"],
    ///   ["E", "exit"],
    ///   ["M", "spawn bruh"]
    /// ];
    /// config.set_keybinds(keybinds);
    /// ```
    pub fn set_keybinds(&mut self, keybinds: Vec<[&str; 2]>) -> &mut Self {
        let keybinds = self.serialize_to_owned(&keybinds);

        for keybind in keybinds {
            self.set_keybind(&keybind[0], &keybind[1]);
        }

        return self;
    }

    /// Every keybind is optional, so you can just provide it with `None` keyword
    ///
    /// # Example
    /// ```
    /// use river_rs::config::Config;
    ///
    /// let mut config = Config::new();
    /// config.set_mouse_keybinds(Some("move-view"), Some("resize-view"), None);
    /// ```
    ///
    pub fn set_mouse_keybinds(
        &mut self,
        left: Option<&str>,
        right: Option<&str>,
        middle: Option<&str>,
    ) -> &mut Self {
        if let Some(left_command) = left {
            self.apply_mouse_keybind("left", left_command);
        }
        if let Some(right_command) = right {
            self.apply_mouse_keybind("right", right_command);
        }
        if let Some(middle_command) = middle {
            self.apply_mouse_keybind("middle", middle_command);
        }
        return self;
    }

    fn apply_mouse_keybind(&self, position: &str, command: &str) {
        let pos: &str;
        match position {
            "left" => {
                pos = "BTN_LEFT";
            }
            "right" => {
                pos = "BTN_RIGHT";
            }
            "middle" => {
                pos = "BTN_MIDDLE";
            }
            _ => {
                pos = "BTN_LEFT";
            }
        }
        Command::new("riverctl")
            .args([
                "map-pointer",
                "normal",
                self.modifier.as_str(),
                pos,
                command,
            ])
            .spawn()
            .expect("Can't set the mouse keybind");
    }

    /// Convenient function to simplify writing config from the end users perspective
    fn serialize_to_owned(&self, arr: &Vec<[&str; 2]>) -> Vec<Vec<String>> {
        let mut new_arr: Vec<Vec<String>> = Vec::new();

        for keybind in arr {
            new_arr.push(vec![String::from(keybind[0]), String::from(keybind[1])])
        }

        return new_arr;
    }

    fn apply_keybind(&self, keybind: Keybind) {
        let command: Vec<&str> = keybind.command.split_whitespace().collect();
        match command.len() {
            1 => {
                Command::new("riverctl")
                    .args([
                        "map",
                        "normal",
                        keybind.modifier.as_str(),
                        keybind.keymap.as_str(),
                        command[0],
                    ])
                    .spawn()
                    .expect("Can't set the keybind\n");
            }
            2 => {
                let args = [
                    "map",
                    "normal",
                    keybind.modifier.as_str(),
                    keybind.keymap.as_str(),
                    command[0],
                    command[1],
                ];
                Command::new("riverctl")
                    .args(args)
                    .spawn()
                    .expect("Can't set the keybind\n");
            }
            0 => {
                panic!("There are no commands provided for the riverctl!\n")
            }
            _ => {
                let args: Vec<&str> = [
                    "map",
                    "normal",
                    keybind.modifier.as_str(),
                    keybind.keymap.as_str(),
                ]
                .iter()
                .chain(&command)
                .map(|&x| x)
                .collect();
                Command::new("riverctl")
                    .args(args)
                    .spawn()
                    .expect("Can't set the keybind\n");
            }
        }
    }

    /// Finish setting up the config.
    ///
    /// Needs to be run at the end of setup via chaining.
    pub fn apply(&mut self) -> Result<()> {
        for keybind in &self.keybinds {
            let keybind = keybind.clone();
            self.apply_keybind(keybind);
        }
        self.apply_colors();
        self.layout.spawn();
        return Ok(());
    }
}
