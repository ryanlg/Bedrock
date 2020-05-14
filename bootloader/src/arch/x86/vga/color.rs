/** All colors VGA text mode supports */
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

/**
 * VGA character color.
 *
 * Defined as two parts: upper 4 bits for background color,
 * lower 4 bits for foreground color.
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TextColor(u8);
impl TextColor {
    pub fn new(fg: Color, bg: Color) -> Self {
        TextColor( ((bg as u8) << 4) | (fg as u8))
    }

    pub fn set_foreground_color(&mut self, fg: Color) {
        self.0 = (self.0 & 0xF0) | (fg as u8);
    }

    pub fn set_background_color(&mut self, bg: Color) {
        self.0 = (self.0 & 0x0F) | ((bg as u8) << 4);
    }
}
