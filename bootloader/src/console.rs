/** Empty trait, just so we can have it as a type in Console trait */
pub trait ConsoleColor {}

pub trait Console<Color>
        where Color: ConsoleColor {

    /* Foreground and background color */
    fn set_foreground_color(&self, color: &Color);
    fn set_background_color(&self, color: &Color);

    /* Print a sequence of bytes */
    fn print_bytes(&self, bytes: &[u8]);

    /* Print just one byte */
    fn print_byte(&self, byte: u8);

    /* Just print a new line */
    fn print_newline(&self);

    /* Print a sequence of bytes, end with a newline */
    fn println_bytes(&self, bytes: &[u8]);
}
