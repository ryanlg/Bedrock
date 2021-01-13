pub const VGA_TEXT_BUFFER_ADDRESS: i32    = 0xB8000;
pub const VGA_TEXT_BUFFER_WIDTH:   usize  = 80;
pub const VGA_TEXT_BUFFER_HEIGHT:  usize  = 25;

// Taken from
// http://www.scs.stanford.edu/17wi-cs140/pintos/specs/freevga/vga/attrreg.htm
// Address of Input Status #1 Register
pub const VGA_TEXT_ATTR_STATUS_REG_ADDR:    u16  = 0x03DA;

// Address of Address Register
pub const VGA_TEXT_ATTR_ADDR_REG_ADDR:      u16  = 0x03C0;

// Address of writing to Data Register
pub const VGA_TEXT_ATTR_DATA_WR_REG_ADDR:   u16 = 0x03C0;

// Address of reading from Data Register
pub const VGA_TEXT_ATTR_DATA_RD_REG_ADDR:   u16 = 0x03C1;

// Index of Attribute Mode Control Register
pub const VGA_TEXT_ATTR_MODE_CONTROL_INDEX: u8  = 0x10;
