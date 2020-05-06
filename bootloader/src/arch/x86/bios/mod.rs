use crate::constants;

pub mod bda {
    pub unsafe fn get_word_at_offset(offset: u16) -> u16 {

        let bda_base = constants::BDA_BASE;
        let addr = (bda_base + offset) as *const u16;

        *addr
    }
}
