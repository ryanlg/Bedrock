pub trait SerialConsole {
    fn print(&self, s: &[u8]);

    fn println(&self);
}
