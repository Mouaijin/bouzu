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

pub fn high_nibble(val: u8) -> u8 {
    (val & 0b11110000) >> 4
}
pub fn low_nibble(val: u8) -> u8 {
    val & 0b00001111
}

pub fn swap8(val: &mut u8) {
    let high = high_nibble(*val);
    let lo = low_nibble(*val);
    *val = (lo << 4) | high;
}

pub fn swap16(val: u16) -> u16 {
    let (hi, lo) = split_u16(val);
    join_u8(lo, hi)
}
///adds with wrap, return carry and half carry
pub fn add(u0 : u8, u1 : u8) -> (u8,bool,bool){
    let sum = u0 as u16 + u1 as u16;
    //todo
}


pub type Addr = u16;
pub type Du8 = u8;
pub type Ds8 = i8;
pub type BitIndex = u8;
pub type Du16 = u16;
