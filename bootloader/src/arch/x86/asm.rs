use core::ptr::read_volatile;

/** Assembly linkage */

#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn outb(addr: u16, byte: u8) {
    llvm_asm!("out dx, al"
              :
              : "{dx}"(addr), "{al}"(byte)
              :
              : "intel")
}

#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn inb(addr: u16) -> u8 {
    let byte: u8;
    llvm_asm!("in al, dx"
              : "={al}"(byte)
              : "{dx}"(addr)
              :
              : "intel");

    // In situation where we don't need the return value,
    // Rust will try to optimize away this call altogether
    // even though we are basically writing assembly directly,
    // which is ridiculous.
    //
    // Mark the byte as volatile to workaround the optimization.
    read_volatile(&byte)
}
