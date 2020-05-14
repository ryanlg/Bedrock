use crate::arch::x86::bios::bda;
use crate::arch::x86::asm::{outb, inb};

use crate::console::{ConsoleColor, Console};

/**
* Since in BIOS's BDA there's only 4 ports defined, we can parameterize them
*/
#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Port {
    First    = 0,
    Second   = 2,
    Third    = 4,
    Fourth   = 6,
}

/** @incomplete: place holder for now */
pub enum SerialConsoleColor { }
impl ConsoleColor for SerialConsoleColor {}
type SerialColor = SerialConsoleColor;

/**
* Implementes the SerialConsole trait, use a BIOS COM port to print
* to the serial port
*/
pub struct SerialConsole {
    serial_port: SerialPort,
}


/**
* Represents a serial port provided to us by the BIOS
* It holds an address - supposedly the address in the BDA (or set manually)
* that we can use `outb` to output characters to.
*/
#[derive(Copy, Clone)]
#[repr(transparent)]
struct SerialPort(u16);

impl SerialPort {

    unsafe fn write_byte_at_offset(&self, offset: u16, byte: u8) {
        outb(self.0 + offset, byte);
    }

    unsafe fn init(&self) -> bool {

        if !self.is_valid() { return false; }

        // Now we initialize those ports
        self.write_byte_at_offset(1, 0x00);   // Disable all interrupts
        self.write_byte_at_offset(3, 0x80);   // Enable DLAB to set baud rate
        self.write_byte_at_offset(0, 0x03);   // Set divisor's low byte to 3 -> 38400 baud
        self.write_byte_at_offset(1, 0x00);   // Set divisor's high byte to 0
        self.write_byte_at_offset(3, 0x03);   // 8 bits, no parity, one stop bit
        self.write_byte_at_offset(2, 0xC7);   // Enable FIFO, clear them with a 14-byte threshold
        self.write_byte_at_offset(4, 0x0B);   // IRQs enabled, RTS/DSR set

        true
    }

    #[inline(always)]
    unsafe fn is_valid(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    unsafe fn tx_byte(&self, byte: u8) -> bool{

        if !self.is_valid() { return false; }

        self.write_byte_at_offset(0, byte);
        true
    }

    #[inline(always)]
    unsafe fn tx_bytes(&self, bytes: &[u8]) -> bool{

        if !self.is_valid() { return false; }

        for &byte in bytes {
            while !self.is_tx_empty() {}
            self.tx_byte(byte);
        }

        true
    }

    #[inline(always)]
    unsafe fn is_tx_empty(&self) -> bool {
        inb(self.0 + 5) & 0x20 != 0
    }
}

impl SerialConsole {
    pub fn new(port: Port) -> Self {
        unsafe {
            // 0x40 is the first COM port
            let addr = bda::get_word_at_offset(port as u16);
            let port = SerialPort(addr);

            port.init();

            SerialConsole {
                serial_port: port
            }
        }
    }

}

impl Console<SerialColor> for SerialConsole {

    // @incomplete
    fn set_foreground_color(&mut self, _color: &SerialColor) {}
    fn set_background_color(&mut self, _color: &SerialColor) {}

    /* Print a sequence of bytes */
    fn print_bytes(&mut self, bytes: &[u8]) {
        unsafe {
            self.serial_port.tx_bytes(bytes);
        }
    }

    /* Print just one byte */
    fn print_byte(&mut self, byte: u8) {
        unsafe {
            self.serial_port.tx_byte(byte);
        }
    }

    /* Print a sequence of bytes, end with a newline */
    fn println_bytes(&mut self, bytes: &[u8]) {
        self.print_bytes(bytes);
        self.print_newline();
    }

    /* Print just a new line */
    fn print_newline(&mut self) {
        unsafe {
            self.serial_port.tx_byte(13);  // \r
            self.serial_port.tx_byte(10);  // \n
        }
    }

    /* You can't clear a serial output */
    fn clear(&mut self) { }
}
