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
    byte
}
