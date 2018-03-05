pub fn split_u16(val : u16) -> (u8,u8){
    let hi = (val >> 8) as u8;
    let lo = ((val << 8) >> 8) as u8;
    (hi, lo)
}
pub fn join_u8(hi : u8, lo : u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}
pub type Addr = u16;
pub type Du8 = u8;
pub type Ds8 = i8;
pub type BitIndex = u8;
pub type Du16 = u16;