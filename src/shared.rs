///returns (hi,lo)
pub fn split_u16(val: u16) -> (u8, u8) {
    let hi = (val >> 8) as u8;
    let lo = ((val << 8) >> 8) as u8;
    (hi, lo)
}

pub fn join_u8(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}

pub fn nth_bit(num: u8, bit_index: u8) -> bool {
    if bit_index > 7 || bit_index < 0 {
        panic!("you literally can't read bits outside of a byte");
    }
    //shift, mask, and compare
    (num >> bit_index) & 1 == 1
}

pub type Addr = u16;
pub type Du8 = u8;
pub type Ds8 = i8;
pub type BitIndex = u8;
pub type Du16 = u16;
