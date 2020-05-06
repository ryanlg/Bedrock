use super::bios::bda;
use super::cpu::asm::{outb, inb};

use crate::serial::SerialPort;

/**
 */
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SerialPort(u16);

impl SerialPort {
    unsafe fn write_byte_at_offset(&self, offset: u16, byte: u8) {
        outb(self.0 + offset, byte);
    }

    unsafe fn write_byte_at_offset(&self, offset: u16, byte: u8) {
        outb(self.0 + offset, byte);
    }
}

impl SerialPort for SerialPort {

    pub fn init(&self) -> bool {

        if !port.is_valid() 
            return false;

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
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub unsafe fn tx_byte(&self, byte: u8) -> bool{

        if !port.is_valid() return false;

        self.write_byte_at_offset(0, byte);
        true
    }

    #[inline(always)]
    pub unsafe fn tx_bytes(&self, bytes: &[u8]) -> bool{

        if !port.is_valid() return false;

        for &byte in bytes {
            while !self.is_tx_empty() {}
            self.write_byte_at_offset(0, byte);
        }

        true
    }

    #[inline(always)]
    pub unsafe fn is_tx_empty(&self) -> bool {
        inb(self.0 + 5) & 0x20 != 0
    }
}

impl Serial {

    /**
     * Create a new Seiral representation of the COM ports defined in BDA
     */
    pub unsafe fn new() -> Self {

        let mut devices = [SerialPort(0); BDA_COM_NUM];

        for index in 0..BDA_COM_NUM {

            let addr = bda::get_word_at_offset(index as u16);
            let port = SerialPort(addr);
            devices[index] = port;

        }

        Serial {
            devices: devices,
        }
    }
}
