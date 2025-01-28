use crate::colors::Colors;
use std::process::Command;

/// Struct for holding pairs of keymap and associated command
/// # Example
/// ```
/// let keymap = Keybind {keymap: String::from("Q"), command: String::from("spawn foo")};
/// ```
/// You should not write Keybinds by yourself, that's why `set_keybind` and `set_keybinds` are for
#[derive(Clone, Debug)]
pub struct Keybind {
    keymap: String,
    command: String,
}

/// The heart of the River configuration, holding values for default and mouse-related keybinds, colors and modifier
/// Each value can only be changed with methods, because it is Rust, baby!
/// For more info, check associated structs and implemented methods.
#[derive(Debug)]
pub struct Config {
    keybinds: Vec<Keybind>,
    mouse: Vec<Keybind>,
    colors: Colors,
    modifier: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            keybinds: vec![],
            mouse: vec![],
            colors: Colors::default(),
            modifier: String::from("Super"),
        }
    }

    /// Sets xkb settings related to repeat_rate and repeat_delay.
    ///
    /// The typematic delay indicates the amount of time (typically in milliseconds) a key needs to be pressed and held in order for the repeating process to begin.
    /// After the repeating process has been triggered, the character will be repeated with a certain frequency (usually given in Hz) specified by the typematic rate.
    /// (Taken from the Arch Wiki)
    pub fn set_repeat(&self, repeat_rate: i32, repeat_delay: i32) -> &Self {
        Command::new("riverctl")
            .args([
                "-set-repeat",
                repeat_rate.to_string().as_str(),
                repeat_delay.to_string().as_str(),
            ])
            .spawn()
            .expect("Can't set xkb settings");

        return self;
    }

    /// Changes the River Modifier key, useful when chaining `set_keybinds`
    /// with different modifiers
    pub fn change_super(&mut self, key: &str) -> &mut Self {
        self.modifier = String::from(key);
        return self;
    }

    /// Sets the single keybind
    ///
    /// # Example
    /// ```
    /// let mut config = Config::new();
    /// let key = String::from("Q");
    /// let command = String::from("spanw foo");
    /// config.set_keybind(&key, &command);
    /// ```
    pub fn set_keybind(&mut self, keys: &String, command: &String) -> &mut Self {
        let keys = keys.clone();
        let command = command.clone();

        let keybind = Keybind {
            keymap: keys,
            command,
        };

        self.keybinds.push(keybind);
        return self;
    }

    /// Sets keybinds based on the vector of lists with 2 values
    /// Takes the ownership of the vector and modifies it to supply `riverctl`
    ///
    /// # Examples
    /// ```
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

    /// Convenient function to simplify writing config from the end users perspective
    fn serialize_to_owned(&self, arr: &Vec<[&str; 2]>) -> Vec<Vec<String>> {
        let mut new_arr: Vec<Vec<String>> = Vec::new();

        for keybind in arr {
            new_arr.push(vec![String::from(keybind[0]), String::from(keybind[1])])
        }

        return new_arr;
    }
}
