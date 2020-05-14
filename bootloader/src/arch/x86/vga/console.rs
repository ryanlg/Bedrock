use crate::console::{Console, ConsoleColor};

use crate::arch::x86::asm::{inb, outb};
use crate::arch::x86::constants::*;
use crate::arch::x86::vga::text::TextBuffer;
use crate::arch::x86::vga::color::{Color, TextColor};

// @incomplete
impl ConsoleColor for Color {}

pub struct VgaConsole {
    buffer:  TextBuffer,
    color:   TextColor,

    // Current column and row to print characters to
    col:     usize,
    row:     usize,
}

impl VgaConsole {
    pub fn new(clear: bool) -> Self {

        let mut console = VgaConsole {
            buffer:   TextBuffer::new(),
            color:    TextColor::new(Color::White, Color::Black),
            col:      0,
            row:      0,
        };

        if clear {
            console.clear();
        }

        // Disable blink by default
        console.disable_blink();

        console
    }

    #[inline(always)]
    fn try_new_line(&mut self) {

        if self.col >= VGA_TEXT_BUFFER_WIDTH {
            self.col = 0;
            self.row += 1;
            self.try_scroll_up();
        }
    }

    #[inline(always)]
    fn try_scroll_up(&mut self) {
        if self.row >= VGA_TEXT_BUFFER_HEIGHT {
            self.buffer.scroll_up();
        }
    }

    /* Advance the cursor by one, new line and scroll up if needed */
    fn advance(&mut self) {
        self.col += 1;

        self.try_new_line();
    }

    /* Disable blinking
     * Apprently QEMU by default disables blinking while Bochs enables it,
     * to unify the output, we disable the blinking unconditionally
     * */
    fn disable_blink(&self) {

        unsafe {
            // Taken from
            // http://www.scs.stanford.edu/17wi-cs140/pintos/specs/freevga/vga/vgareg.htm
            // Read the value from status reg and discard it 
            let _ = inb(VGA_TEXT_ATTR_STATUS_REG_ADDR);

            // Save old data register value, need to restore later
            let old_data_reg_value = inb(VGA_TEXT_ATTR_ADDR_REG_ADDR);

            // Write the index of Mode Control register
            outb(VGA_TEXT_ATTR_ADDR_REG_ADDR, VGA_TEXT_ATTR_MODE_CONTROL_INDEX);

            // Read from the register
            let mut mode_control = inb(VGA_TEXT_ATTR_DATA_RD_REG_ADDR);

            // Clear Blink bit (3)
            mode_control &= 0xF7;

            // Write it back
            outb(VGA_TEXT_ATTR_DATA_WR_REG_ADDR, mode_control);

            // Write old value back
            outb(VGA_TEXT_ATTR_ADDR_REG_ADDR, old_data_reg_value);
        }
    }
}

impl Console<Color> for VgaConsole {

    fn set_foreground_color(&mut self, color: Color) {
        self.color.set_foreground_color(color);
    }

    fn set_background_color(&mut self, color: Color) {
        self.color.set_background_color(color);
    }

    fn print_byte(&mut self, byte: u8) {
        self.buffer.write_cell(self.row, self.col, byte, self.color);
        self.advance();
    }

    fn print_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.print_byte(byte);
        }
    }

    fn print_newline(&mut self) {
        self.row += 1;
        self.col  = 0;
        self.try_scroll_up();
    }

    fn println_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.print_byte(byte);
        }

        self.print_newline();
    }

    fn clear(&mut self) {
        self.buffer.clear(self.color);
    }
}
