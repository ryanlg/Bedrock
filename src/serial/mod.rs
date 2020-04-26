use crate::constants::BDA_COM_NUM;
use crate::bios::bda;
use crate::cpu::asm::{outb, inb};

/**
 * Represents individual seiral ports with an address
 */
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SerialPort(u16);

impl SerialPort {

    pub fn is_valid(self) -> bool {
        self.0 != 0
    }

    pub unsafe fn write_byte_at_offset(self, offset: u16, byte: u8) {
        outb(self.0 + offset, byte);
    }

    #[inline(always)]
    unsafe fn is_output_ready(self) -> bool {
        inb(self.0 + 5) & 0x20 != 0
    }
}

/**
 * Collection of serial ports predefined by the BIOS's BDA
 */
#[repr(C)]
pub struct Serial {
    devices: [SerialPort; BDA_COM_NUM]
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

            if !port.is_valid() { continue; }

            // Now we initialize those ports
            port.write_byte_at_offset(1, 0x00);   // Disable all interrupts
            port.write_byte_at_offset(3, 0x80);   // Enable DLAB to set baud rate
            port.write_byte_at_offset(0, 0x03);   // Set divisor's low byte to 3 -> 38400 baud
            port.write_byte_at_offset(1, 0x00);   // Set divisor's high byte to 0
            port.write_byte_at_offset(3, 0x03);   // 8 bits, no parity, one stop bit
            port.write_byte_at_offset(2, 0xC7);   // Enable FIFO, clear them with a 14-byte threshold
            port.write_byte_at_offset(4, 0x0B);   // IRQs enabled, RTS/DSR set
        }

        Serial {
            devices: devices,
        }
    }

    /** Write a byte to a COM port */
    pub unsafe fn broadcast_byte(self, byte: u8) {
        for port in self.devices.iter() {

            if !port.is_valid() { continue; }

            // Wait for the output buffer to be ready
            while !port.is_output_ready() {}

            // Now we write the byte
            port.write_byte_at_offset(0, byte);
        }
    }
}
