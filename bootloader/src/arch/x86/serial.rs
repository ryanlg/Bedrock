use super::bios::bda;
use super::asm::{outb, inb};

use crate::serial::SerialConsole;

/**
 * Represents a serial port provided to us by the BIOS
 * It holds an address - supposedly the address in the BDA (or set manually)
 * that we can use `outb` to output characters to.
 */
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BiosSerialPort(u16);

impl BiosSerialPort {
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

/**
 * Implementes the SerialConsole trait, use a BIOS COM port to print
 * to the serial port
 */
pub struct BiosSerialConsole {
    serial_port: BiosSerialPort,
}

impl BiosSerialConsole {
    pub fn new(offset: u8) -> Self {
        unsafe {
            // 0x40 is the first COM port
            let addr = bda::get_word_at_offset(offset as u16);
            let port = BiosSerialPort(addr);

            port.init();

            BiosSerialConsole {
                serial_port: port
            }
        }
    }
}

impl SerialConsole for BiosSerialConsole {

    fn print(&self, bytes: &[u8]) {
        unsafe {
            self.serial_port.tx_bytes(bytes);
        }
    }

    fn println(&self) {
        unsafe {
            self.serial_port.tx_byte(13);  // \r
            self.serial_port.tx_byte(10);  // \n
        }
    }
}
