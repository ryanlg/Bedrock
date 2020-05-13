use super::constants::*;

#[repr(u8)]
#[allow(dead_code)]
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
#[derive(Copy, Clone)]
pub struct TextColor(u8);
impl TextColor {
    pub fn new(fg: Color, bg: Color) -> Self {
        TextColor( ((bg as u8) << 4) | (fg as u8))
    }
}

/** Represents a two byte VGA character */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Character {
    ascii: u8,
    color: TextColor,
}

impl Character {
    pub fn new(a: u8, c: TextColor) -> Self {
        Character {
            ascii: a,
            color: c,
        }
    }
}

/**
 * Represents the video memory
 */
type CharacterSquare = [[Character; VGA_TEXT_BUFFER_WIDTH]; VGA_TEXT_BUFFER_HEIGHT];
pub struct TextBuffer {
    characters: &'static mut CharacterSquare,
}

impl TextBuffer {
    pub fn new() -> Self {
        unsafe {
            TextBuffer {
                characters: &mut *(VGA_TEXT_BUFFER_ADDRESS as *mut CharacterSquare)
            }
        }
    }

    #[inline(always)]
    pub fn write_cell(&mut self, row: usize, col: usize, c: Character) {
        self.characters[row][col] = c;
    }
}

