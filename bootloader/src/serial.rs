pub trait SerialPort {

    unsafe fn init(&self)     -> bool;
    unsafe fn is_valid(&self) -> bool;

    unsafe fn tx_byte(&self,  byte: u8)     -> bool;
    unsafe fn tx_bytes(&self, bytes: &[u8]) -> bool;

    unsafe fn is_tx_empty(&self)  -> bool;
}
