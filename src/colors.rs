//! # Colors
//!
//! Colors struct is used for describing River basic color palette
//!
//! Often colors are overriden by tiling generators (like wideriver),
//! or by background image apps (by overlapping WM's background),
//! so you can use defaults, specified in the `init` file, or go for
//! your own settings

#[derive(Debug)]
pub struct Colors {
    pub background_color: u32,
    pub border_color_focused: u32,
    pub border_color_unfocused: u32,
}

impl Colors {
    /// Set next values as default:
    ///
    /// ```
    /// let background_color: u32 = 0x002b36;
    /// let border_color_focused: u32 = 0x93a1a1;
    /// let border_color_unfocused: u32 = 0x586e75;
    /// ```
    pub fn default() -> Colors {
        Colors {
            background_color: 0x002b36,
            border_color_focused: 0x93a1a1,
            border_color_unfocused: 0x586e75,
        }
    }

    pub fn set_background_color(&mut self, color: u32) -> &mut Self {
        self.background_color = color;
        return self;
    }

    pub fn set_border_color_focused(&mut self, color: u32) -> &mut Self {
        self.border_color_focused = color;
        return self;
    }

    pub fn set_border_color_unfocused(&mut self, color: u32) -> &mut Self {
        self.border_color_unfocused = color;
        return self;
    }
}
