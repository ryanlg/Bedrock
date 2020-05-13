/** Base address of the BDA */
const BDA_BASE: u16 = 0x0400;

pub unsafe fn get_word_at_offset(offset: u16) -> u16 {

    let bda_base = BDA_BASE;
    let addr = (bda_base + offset) as *const u16;

    *addr
}
