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

    fn print_byte(&mut self, byte: u8) {
        self.serial_console.print_byte(byte);
        self.vga_console.print_byte(byte);
    }

    fn print_bytes(&mut self, bytes: &[u8]) {
        self.serial_console.print_bytes(bytes);
        self.vga_console.print_bytes(bytes);
    }

    fn print_newline(&mut self) {
        self.serial_console.print_newline();
        self.vga_console.print_newline();
    }

    fn println_bytes(&mut self, bytes: &[u8]) {
        self.serial_console.println_bytes(bytes);
        self.vga_console.println_bytes(bytes);
    }

    fn clear(&mut self) {
        self.vga_console.clear();
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
