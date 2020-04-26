// We can't use any std:: stuff, have to create our own
#![no_std]

// Enable inline assembly
#![feature(llvm_asm)]

// Required to pass compilation
#![feature(lang_items)]


// ================ Modules ================
mod bios;
mod cpu;
mod constants;
mod serial;


// ================ Imports ================
use core::panic::PanicInfo;

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
pub extern "C" fn _kernel_entry() -> ! {

    unsafe {
        let serial = serial::Serial::new();
        serial.broadcast_byte('a' as u8);
    }

    loop {}
}
