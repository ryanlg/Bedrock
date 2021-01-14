// We can't use any std:: stuff, have to create our own
#![no_std]

// Enable inline assembly
#![feature(llvm_asm)]

// Required to pass compilation
#![feature(lang_items)]

// Needed for some MaybeUninit APIs
#![feature(maybe_uninit_extra)]

// Import intrinsics, we are not going to use them directly
// but Rust code will try to link to these functions
extern crate intrinsics;

// ================ Modules ================
mod arch;

mod core;
mod console;
mod panic;

// ================ Imports ================
use crate::core::Core;


#[no_mangle]
#[cfg(target_arch = "x86")]
#[link_section = ".rust_text"]
extern "C" fn _bootloader_entry() -> ! {

    use crate::arch::x86::core::Core;
    use crate::console::Console;

    core!().get_console().clear();
    print!("TESTING");

    loop {}
}
