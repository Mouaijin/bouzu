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
pub fn add(u0 : u8, u1 : u8, c : bool) -> (u8,bool,bool){
    let sum = u0 as u16 + u1 as u16 + c as u16;
    let carry = sum > 0xffff;
    let half = nth_bit(4, low_nibble(u0) + low_nibble(u1) + c as u8);
    (sum as u8, carry, half)
    //todo
}
///subs with wrap, return carry and half carry
pub fn sub(u0 : u8, u1 : u8, c : bool) -> (u8,bool,bool){
    let sub = u0 as i16 - u1 as i16 - c as i16;
    let carry = sub < 0;
    let half = (low_nibble(u0) as i16 - low_nibble(u1) as i16 - c as i16) < 0;
    (sub as u8, carry, half)
    //todo
}


pub type Addr = u16;
pub type Du8 = u8;
pub type Ds8 = i8;
pub type BitIndex = u8;
pub type Du16 = u16;
