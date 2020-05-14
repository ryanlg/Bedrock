/** Empty trait, just so we can have it as a type in Console trait */
pub trait ConsoleColor {}

pub trait Console<Color>
        where Color: ConsoleColor {

    /* Foreground and background color - if supported */
    fn set_foreground_color(&mut self, color: &Color);
    fn set_background_color(&mut self, color: &Color);

    /* Print a sequence of bytes */
    fn print_bytes(&mut self, bytes: &[u8]);

    /* Print just one byte */
    fn print_byte(&mut self, byte: u8);

    /* Just print a new line */
    fn print_newline(&mut self);

    /* Print a sequence of bytes, end with a newline */
    fn println_bytes(&mut self, bytes: &[u8]);

    /* Clear the screen - if supported */
    fn clear(&mut self);
}
