// We can't use any std:: stuff, have to create our own
#![no_std]

// Enable inline assembly
#![feature(llvm_asm)]

// Required to pass compilation
#![feature(lang_items)]


// ================ Modules ================
mod arch;

mod serial;

// ================ Imports ================
use core::panic::PanicInfo;

#[cfg(target_arch = "x86")]
use crate::arch::x86;

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
extern "C" fn _bootloader_entry() -> ! {
    loop {}
}
