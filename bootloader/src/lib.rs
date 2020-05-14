// We can't use any std:: stuff, have to create our own
#![no_std]

// Enable inline assembly
#![feature(llvm_asm)]

// Required to pass compilation
#![feature(lang_items)]

// Import intrinsics, we are not going to use them directly
// but Rust code will try to link to these functions
extern crate intrinsics;

// ================ Modules ================
mod arch;

mod console;

// ================ Imports ================
use core::panic::PanicInfo;

use crate::console::{Console};



/*
 * Rust requires a panic handler and a eh_personality lang item.
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn handle_error() {
    loop {}
}

#[no_mangle]
#[cfg(target_arch = "x86")]
#[link_section = ".rust_text"]
extern "C" fn _bootloader_entry() -> ! {

    use crate::arch::x86::bios::serial::{SerialConsole, Port};
    use crate::arch::x86::vga::color::Color;
    use crate::arch::x86::vga::console::VgaConsole;

    let mut vga_console = VgaConsole::new(false);
    vga_console.set_background_color(&Color::Yellow);
    vga_console.set_foreground_color(&Color::Green);
    vga_console.println_bytes(b"TESTIING");

    /*
    let mut console = SerialConsole::new(Port::First);
    console.print_newline();
    console.println_bytes(b"TESTING");
    */

    loop {}
}
