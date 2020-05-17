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
