//! # Layout
//!
//! Basic struct for holding RiverWM layout generator and its arguments.
//! For more complex layout generator configurations consider using multiline
//! strings.
use std::process::Command;

#[derive(Debug)]
pub struct Layout {
    generator: String,
    arguments: String,
}

impl Layout {
    pub fn default() -> Layout {
        Layout {
            generator: String::from("rivertile"),
            arguments: String::from("-view-padding 6 -outer-padding 6"),
        }
    }

    pub fn set_generator(&mut self, generator: &str) -> &mut Self {
        self.generator = String::from(generator);
        return self;
    }

    /// Consider writing args in multiline strings
    /// # Example
    /// ```
    /// use river_rs::layout::Layout;
    /// let mut layout = Layout::default();
    /// let args = "-view-padding 6 \
    ///             -outer-padding 6";
    /// layout.set_generator("rivertile");
    /// layout.set_args(args);
    /// ```
    pub fn set_args(&mut self, args: &str) -> &mut Self {
        self.arguments = String::from(args);
        return self;
    }

    pub fn spawn(&self) {
        let args: Vec<&str> = self.arguments.trim().split_whitespace().collect();

        Command::new("riverctl")
            .args(["default-layout", self.generator.as_str()])
            .spawn()
            .expect("Can't set default layout generator");

        Command::new(self.generator.as_str())
            .args(args)
            .spawn()
            .expect("Can't launch layout generator");
    }
}
