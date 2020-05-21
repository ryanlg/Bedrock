use crate::console;

/// Strcture to share some core information between different
/// components of the bootloader. Implementations must provide
/// ways to access this struct globally by get_global(). This enables
/// some convenient macro.
pub trait Core {

    type ConsoleColor: console::ConsoleColor;
    type Console:      console::Console<Self::ConsoleColor>;

    fn get_console(&mut self) -> &mut Self::Console;
    fn get_global() -> &'static mut Self;
}

#[macro_export]
macro_rules! core {
    () => (Core::get_global())
}

#[macro_export]
macro_rules! println {
    ()            => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use ::core::fmt::Write;
        let _ = core!().get_console().write_fmt(format_args!("{}", ($($arg)*)));
    }
}

