#![no_std]
#![feature(llvm_asm)]

pub mod bios;
pub mod constants;
pub mod instructions;
pub mod vga;

/// We only compiles on x86
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!("x86 crate only supports, you guessed it, x86 arch.");
