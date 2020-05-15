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
