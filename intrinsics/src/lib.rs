#![no_std]

#![feature(global_asm, llvm_asm)]

/*
 * Rudimentary memcpy implementation in Rust
 *
 * No checks on address overlap
 */
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, count: usize) {

    /* Use a plain while statement instead of a fancy for-in to avoid
     * Rust generating an iterator in Debug build, even though it will
     * be optimized away to a plain C loop in a Release build, it still
     * causes Bochs to choke on something weird*/
    let mut i = 0;
    while i < count {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
}

/*
 * Rudimentary memset implementation in Rust
 */
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: char, count: usize) {

    // Here we assuem direction flag is cleared
    llvm_asm!("rep stosb"
              :
              : "{edi}"(dest),
                "{ecx}"(count),
                "{al}"(c)
              : "memory",
                "edi",
                "ecx",
                "al"
              : "intel");
}

/*
 * A fake handle to _Unwind_Resume, which supposedly should be inserted by
 * LLVM (not sure on that), but it ends up missing during linking and
 * causes error in our linker.
 *
 * This is a hacky way to deal with it - though it will be fine since
 * in Release build, LTO will strip away any traces of this, so this is just
 * here to pass the linker in a Debug build. We shouldn't hit this anyways.
 */
global_asm!(r#"
    .global _Unwind_Resume
    _Unwind_Resume:
        ud2
"#);
