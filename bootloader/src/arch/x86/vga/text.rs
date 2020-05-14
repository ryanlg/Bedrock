use crate::arch::x86::constants::*;
use super::color::TextColor;

/** Represents a two byte VGA character */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Character {
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

type CharacterSquare = [[Character; VGA_TEXT_BUFFER_WIDTH]; VGA_TEXT_BUFFER_HEIGHT];

/**
 * Represents the video memory
 */
#[repr(C)]
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

    /* Write a character to a certain location */
    pub fn write_cell(&mut self, row: usize, col: usize,
                                 ascii: u8, color: TextColor) {
        let c = Character {
            ascii: ascii,
            color: color,
        };
        self.characters[row][col] = c;
    }

    /* Scroll the screen up */
    pub fn scroll_up(&mut self) {
        for rowi in 1..self.characters.len() {
            self.characters[rowi - 1] = self.characters[rowi];
        }
    }

    /* Clear the screen with color */
    pub fn clear(&mut self, color: TextColor) {

        let rowc = VGA_TEXT_BUFFER_HEIGHT;
        let colc = VGA_TEXT_BUFFER_WIDTH;

        for rowi in 0..rowc {
            for coli in 0..colc {
                self.characters[rowi][coli] = Character::new(b' ', color);
            }
        }
    }
}
