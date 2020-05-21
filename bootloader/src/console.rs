use core::fmt::Write;

/// Empty trait, just so we can have it as a type in Console trait
pub trait ConsoleColor {}

pub trait Console<Color>: Write
        where Color: ConsoleColor {

    /// Foreground and background color - if supported
    fn set_foreground_color(&mut self, color: Color);
    fn set_background_color(&mut self, color: Color);

    /// Clear the screen - if supported
    fn clear(&mut self);
}
