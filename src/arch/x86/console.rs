use crate::console::{Console, ConsoleColor};

use super::bios::serial::{SerialConsole, Port};
use super::vga::color::Color as VgaColor;
use super::vga::console::VgaConsole;

/* Currently just a copy of VGA color, might change in the future */
#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}
impl ConsoleColor for Color {}

pub struct DualConsole {
    serial_console:  SerialConsole,
    vga_console:     VgaConsole,
}

impl DualConsole {
    pub fn new() -> Self {
        DualConsole {
            serial_console: SerialConsole::new(Port::First),
            vga_console:    VgaConsole::new(true),
        }
    }
}

impl Console<Color> for DualConsole {

    fn set_foreground_color(&mut self, color: Color) {
        self.vga_console.set_foreground_color(VgaColor::from(color));
    }

    fn set_background_color(&mut self, color: Color) {
        self.vga_console.set_background_color(VgaColor::from(color));
    }

    fn clear(&mut self) {
        self.vga_console.clear();
    }
}

impl core::fmt::Write for DualConsole {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {

        // @error_handling
        let _ = self.serial_console.write_str(s);
        let _ = self.vga_console.write_str(s);

        Ok(())
    }
}


impl From<Color> for VgaColor {
    fn from(error: Color) -> Self {
        match error {
            Color::Black       => VgaColor::Black      ,
            Color::Blue        => VgaColor::Blue       ,
            Color::Green       => VgaColor::Green      ,
            Color::Cyan        => VgaColor::Cyan       ,
            Color::Red         => VgaColor::Red        ,
            Color::Magenta     => VgaColor::Magenta    ,
            Color::Brown       => VgaColor::Brown      ,
            Color::LightGray   => VgaColor::LightGray  ,
            Color::DarkGray    => VgaColor::DarkGray   ,
            Color::LightBlue   => VgaColor::LightBlue  ,
            Color::LightGreen  => VgaColor::LightGreen ,
            Color::LightCyan   => VgaColor::LightCyan  ,
            Color::LightRed    => VgaColor::LightRed   ,
            Color::Pink        => VgaColor::Pink       ,
            Color::Yellow      => VgaColor::Yellow     ,
            Color::White       => VgaColor::White      ,
        }
    }
}
