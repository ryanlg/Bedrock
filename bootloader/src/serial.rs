pub trait SeiralPort {

    fn init(&self)     -> bool;
    fn is_valid(&self) -> bool;

    fn tx_byte(&self,  byte: u8)     -> bool;
    fn tx_bytes(&self, bytes: &[u8]) -> bool;

    fn is_tx_empty(&self)  -> bool;
}
