use register::*;
use shared::*;
use mmu;

///```markdown
/// Instruction name format:
/// NameOperandtypeOperandtype
/// Operand types:
///  R8   = 8 bit register
///  R16  = 16 bit register
///  D8   = 8 bit value
///  D16  = 16 bit value
///  A16  = 16 bit address
///  A8   = 8 bit address (offset)
///  AR8  = 8 bit address in register (offset)
///  AR16 = 16 bit address in register
///```
#[derive(Debug)]
pub enum Instruction {
    ///No-op
    Nop,
    ///Halt until interrupt
    Halt,
    ///Halt cpu completely
    Stop,
    ///swap register nibbles
    SwapR8(Reg8Name),
    ///swap address nibbles
    SwapAR16(Reg16Name),
    ///load immediate into 8-bit register
    LdR8D8(Reg8Name, Du8),
    ///load referenced value into 8-bit register
    LdR8A16(Reg8Name, Addr),
    /// Store value in 8 bit register into address
    LdA16R8(Addr, Reg8Name),
    ///Load value in 8 bit register to another register
    LdR8R8(Reg8Name, Reg8Name),
    ///Load 16 bit value into 16 bit register
    LdR16D16(Reg16Name, Du16),
    ///Load value in 16 bit register in 16 bit register
    LdR16R16(Reg16Name, Reg16Name),
    ///Store value in 8 bit register into adress in 16 bit register
    LdAR16R8(Reg16Name, Reg8Name),
    ///Store 8 bit value in address in 16 bit register
    LdAR16D8(Reg16Name, Du8),
    ///Load value in address in 16 bit register into 8 bit register
    LdR8AR16(Reg8Name, Reg16Name),
    ///Store value in 16 bit register into address
    LdA16R16(Addr, Reg16Name),
    ///Store value in 8 bit register into address in 16 bit register and then increment 16 bit register
    LdiAR16R8(Reg16Name, Reg8Name),
    ///Store value in 8 bit register into address in 16 bit register and then decrement 16 bit register
    LddAR16R8(Reg16Name, Reg8Name),
    ///Load value in address in 16 register into 8 bit register and increment 16 bit register
    LdiR8AR16(Reg8Name, Reg16Name),
    ///Load value in address in 16 register into 8 bit register and decrement 16 bit register
    LddR8AR16(Reg8Name, Reg16Name),
    ///Load value in address (FF00 + 8 bit address) in 8 bit register
    LdhR8A8(Reg8Name, Du8),
    ///Store value in 8 bit register in address (FF00 + 8 bit address)
    LdhA8R8(Du8, Reg8Name),
    ///Store value in 8 bit register in address (FF00 + 8 bit register)
    LdhAR8R8(Reg8Name, Reg8Name),
    ///Add signed 8 bit value to SP and copy SP to 16 bit register
    LdhlR16D8(Reg16Name, Ds8),
    ///Inc value in 8 bit register
    IncR8(Reg8Name),
    ///Inc value in 16 bit register
    IncR16(Reg16Name),
    ///Inc value address in 16 bit register
    IncAR16(Reg16Name),
    ///Dec value in 8 bit register
    DecR8(Reg8Name),
    ///Dec value in 16 bit register
    DecR16(Reg16Name),
    ///Dec value address in 16 bit register
    DecAR16(Reg16Name),
    ///Set carry flag
    Scf,
    ///Clear carry flag
    Ccf,
    ///Test bit n in 8 bit register
    BitR8(BitIndex, Reg8Name),
    ///Test bit n in address in 16 bit register
    BitAR16(BitIndex, Reg16Name),
    ///Clear bit n in 8 bit register
    ResR8(BitIndex, Reg8Name),
    ///Clear bit n in address in 16 bit register
    ResAR16(BitIndex, Reg16Name),
    ///Set bit n in 8 bit register
    SetR8(BitIndex, Reg8Name),
    ///Set bit n in address in 16 bit register
    SetAR16(BitIndex, Reg16Name),
    ///Bitwise NOT on register A
    ///Set N(1),H(1)
    Cpl,
    ///Rotate A left with carry
    Rlca,
    ///Rotate A left
    Rla,
    ///Rotate A right with carry
    Rrca,
    ///Rotate A right
    Rra,
    ///Rotate 8 bit register left with carry
    RlcR8(Reg8Name),
    ///Rotate value pointed by 16 bit register left with carry
    RlcAR16(Reg16Name),
    ///Rotate 8 bit register left
    RlR8(Reg8Name),
    ///Rotate value pointed by 16 bit register left
    RlAR16(Reg16Name),
    ///Rotate 8 bit register right with carry
    RrcR8(Reg8Name),
    ///Rotate value pointed by 16 bit register right with carry
    RrcAR16(Reg16Name),
    ///Rotate 8 bit register right
    RrR8(Reg8Name),
    ///Rotate value pointed by 16 bit register right
    RrAR16(Reg16Name),
    ///Shift 8 bit register left, preserving sign
    SlaR8(Reg8Name),
    ///Shift value pointed by 16 bit register left, preserving sign
    SlaAR16(Reg16Name),
    ///Shift 8 bit register right, preserving sign
    SraR8(Reg8Name),
    ///Shift value pointed by 16 bit register right, preserving sign
    SraAR16(Reg16Name),
    ///Shift 8 bit register right
    SrlR8(Reg8Name),
    ///Shift value pointed by 16 bit register right
    SrlAR16(Reg16Name),
    ///Absolute jump to address
    JpA16(Addr),
    ///Jump to address in address in 16 bit register (erhh)
    JpAR16(Reg16Name),
    ///Jump to address if flag is set
    JpFA16(BitFlag, Addr),
    ///Jump to address if flag is not set
    JpNfA16(BitFlag, Addr),
    ///Relative jump with signed offset
    JrA8(Ds8),
    ///Relative jump with signed offset if flag is set
    JrFA8(BitFlag, Ds8),
    ///Relative jump with signed offset if flag is not set
    JrNfA8(BitFlag, Ds8),
    ///Add 8 bit register to 8 bit register
    AddR8R8(Reg8Name, Reg8Name),
    ///Add 8 bit value to 8 bit register
    AddR8D8(Reg8Name, Du8),
    ///Add value pointed by 16 bit register to 8 bit register
    AddR8AR16(Reg8Name, Reg16Name),
    ///Add 16 bit register to 16 bit register
    AddR16R16(Reg16Name, Reg16Name),
    ///Add signed 8 bit value to 16 bit register
    AddR16D8(Reg16Name, Ds8),
    ///Add 8 bit register to 8 bit register with carry
    AdcR8R8(Reg8Name, Reg8Name),
    ///Add 8 bit value to 8 bit register with carry
    AdcR8D8(Reg8Name, Du8),
    ///Add value in address in 16 bit register to 8 bit register with carry
    AdcR8AR16(Reg8Name, Reg16Name),
    ///Subtract value in 8 bit register with 8 bit register
    SubR8R8(Reg8Name, Reg8Name),
    ///Subtract 8 bit value from 8 bit register
    SubR8D8(Reg8Name, Du8),
    ///Subtract value in address in 16 bit register from 8 bit register
    SubR8AR16(Reg8Name, Reg16Name),
    ///Subtract value in 8 bit register + carry from 8 bit register
    SbcR8R8(Reg8Name, Reg8Name),
    ///Subtract value in address in 16 bit register + carry from 8 bit register
    SbcR8AR16(Reg8Name, Reg16Name),
    ///Subtract 8 bit value + carry from 8 bit register
    SbcR8D8(Reg8Name, Du8),
    ///Bitwise AND between 8 bit registers
    AndR8R8(Reg8Name, Reg8Name),
    ///Bitwise AND between 8 bit register and 8 bit value
    AndR8D8(Reg8Name, Du8),
    ///Bitwise AND between 8 bit register and value in address in 16 bit register
    AndR8AR16(Reg8Name, Reg16Name),
    ///Bitwise OR between 8 bit registers
    OrR8R8(Reg8Name, Reg8Name),
    ///Bitwise OR between 8 bit register and 8 bit value
    OrR8D8(Reg8Name, Du8),
    ///Bitwise OR between 8 bit register and value in address in 16 bit register
    OrR8AR16(Reg8Name, Reg16Name),
    ///Bitwise XOR between 8 bit registers
    XorR8R8(Reg8Name, Reg8Name),
    ///Bitwise XOR between 8 bit register and 8 bit value
    XorR8D8(Reg8Name, Du8),
    ///Bitwise XOR between 8 bit register and value in address in 16 bit register
    XorR8AR16(Reg8Name, Reg16Name),
    ///Enabled interrupts
    Ei,
    ///Disable interrupts
    Di,
    ///Compare 8 bit register with 8 bit register
    CpR8R8(Reg8Name, Reg8Name),
    ///Compare 8 bit register with value in address in 16 bit register
    CpR8AR16(Reg8Name, Reg16Name),
    ///Compare 8 bit register with 8 bit value
    CpR8D8(Reg8Name, Du8),
    ///Converts 8 bit register into packed BCD
    DaaR8(Reg8Name),
    ///Push 16 bit register onto stack
    PushR16(Reg16Name),
    ///Pop 16 bit value from stack into 16 bit register
    PopR16(Reg16Name),
    ///Call routine at address
    CallA16(Addr),
    ///Call routine at address if flag is set
    CallFA16(BitFlag, Addr),
    ///Call routine at address if flag is not set
    CallNfA16(BitFlag, Addr),
    ///Return from subroutine
    Ret,
    ///Return from subroutine and enable interrupts
    Reti,
    ///Return if flag is set
    RetF(BitFlag),
    ///Return if flag is not set
    RetNf(BitFlag),
    ///Restart at address (simple call to address)
    Rst(Addr),
}

pub fn decode(mmu: &mmu::Mmu, addr: Addr) -> Instruction {
    //op-code is first byte
    let op = mmu.read8(addr);
    //op-code may be followed by 01 byte arguments
    let arg8_0 = mmu.read8(addr + 1);
    // let arg8_1 = mmu.read8(addr + 2);
    //or op-code may be followed by 01 2byte words
    let arg16 = mmu.read16(addr + 1);
    {
        use self::Instruction::*;
        use register::Reg8Name::*;
        use register::Reg16Name::*;
        match op {
            0x00 => Nop,
            0x01 => LdR16D16(BC, arg16),
            0x02 => LdAR16R8(BC, A),
            0x03 => IncR16(BC),
            0x04 => IncR8(B),
            0x05 => DecR8(B),
            0x06 => LdR8D8(B, arg8_0),
            0x07 => Rlca,
            0x08 => LdA16R16(arg16, SP),
            0x09 => AddR16R16(HL, BC),
            0x0A => LdR8AR16(A, BC),
            0x0B => DecR16(BC),
            0x0C => IncR8(C),
            0x0D => DecR8(C),
            0x0E => LdR8D8(C, arg8_0),
            0x0F => Rrca,
            0x10 => Stop,
            0x11 => LdR16D16(DE, arg16),
            0x12 => LdAR16R8(DE, A),
            0x13 => IncR16(DE),
            0x14 => IncR8(D),
            0x15 => DecR8(D),
            0x16 => LdR8D8(D, arg8_0),
            0x17 => Rla,
            0x18 => JrA8(arg8_0 as i8),
            0x19 => AddR16R16(HL, DE),
            0x1A => LdR8AR16(A, DE),
            0x1B => DecR16(DE),
            0x1C => IncR8(E),
            0x1D => DecR8(E),
            0x1E => LdR8D8(E, arg8_0),
            0x1F => Rra,
            0x20 => JrNfA8(BitFlag::Z, arg8_0 as i8),
            0x21 => LdR16D16(HL, arg16),
            0x22 => LdiAR16R8(HL, A),
            0x23 => IncR16(HL),
            0x24 => IncR8(H),
            0x25 => DecR8(H),
            0x26 => LdR8D8(H, arg8_0),
            0x27 => DaaR8(A),
            0x28 => JrFA8(BitFlag::Z, arg8_0 as i8),
            0x29 => AddR16R16(HL, HL),
            0x2A => LdiR8AR16(A, HL),
            0x2B => DecR16(HL),
            0x2C => IncR8(L),
            0x2D => DecR8(L),
            0x2E => LdR8D8(L, arg8_0),
            0x2F => Cpl,
            0x30 => JrNfA8(BitFlag::C, arg8_0 as i8),
            0x31 => LdR16D16(SP, arg16),
            0x32 => LddAR16R8(HL, A),
            0x33 => IncR16(SP),
            0x34 => IncAR16(HL),
            0x35 => DecAR16(HL),
            0x36 => LdAR16D8(HL, arg8_0),
            0x37 => Scf,
            0x38 => JrFA8(BitFlag::C, arg8_0 as i8),
            0x39 => AddR16R16(HL, SP),
            0x3A => LddR8AR16(A, HL),
            0x3B => DecR16(SP),
            0x3C => IncR8(A),
            0x3D => DecR8(A),
            0x3E => LdR8D8(A, arg8_0),
            0x3F => Ccf,
            0x40 => LdR8R8(B, B),
            0x41 => LdR8R8(B, C),
            0x42 => LdR8R8(B, D),
            0x43 => LdR8R8(B, E),
            0x44 => LdR8R8(B, H),
            0x45 => LdR8R8(B, L),
            0x46 => LdR8AR16(B, HL),
            0x47 => LdR8R8(B, A),
            0x48 => LdR8R8(C, B),
            0x49 => LdR8R8(C, C),
            0x4A => LdR8R8(C, D),
            0x4B => LdR8R8(C, E),
            0x4C => LdR8R8(C, H),
            0x4D => LdR8R8(C, L),
            0x4E => LdR8AR16(C, HL),
            0x4F => LdR8R8(C, A),
            0x50 => LdR8R8(D, B),
            0x51 => LdR8R8(D, C),
            0x52 => LdR8R8(D, D),
            0x53 => LdR8R8(D, E),
            0x54 => LdR8R8(D, H),
            0x55 => LdR8R8(D, L),
            0x56 => LdR8AR16(D, HL),
            0x57 => LdR8R8(D, A),
            0x58 => LdR8R8(E, B),
            0x59 => LdR8R8(E, C),
            0x5A => LdR8R8(E, D),
            0x5B => LdR8R8(E, E),
            0x5C => LdR8R8(E, H),
            0x5D => LdR8R8(E, L),
            0x5E => LdR8AR16(E, HL),
            0x5F => LdR8R8(E, A),
            0x60 => LdR8R8(H, B),
            0x61 => LdR8R8(H, C),
            0x62 => LdR8R8(H, D),
            0x63 => LdR8R8(H, E),
            0x64 => LdR8R8(H, H),
            0x65 => LdR8R8(H, L),
            0x66 => LdR8AR16(H, HL),
            0x67 => LdR8R8(H, A),
            0x68 => LdR8R8(L, B),
            0x69 => LdR8R8(L, C),
            0x6A => LdR8R8(L, D),
            0x6B => LdR8R8(L, E),
            0x6C => LdR8R8(L, H),
            0x6D => LdR8R8(L, L),
            0x6E => LdR8AR16(L, HL),
            0x6F => LdR8R8(L, A),
            0x70 => LdAR16R8(HL, B),
            0x71 => LdAR16R8(HL, C),
            0x72 => LdAR16R8(HL, D),
            0x73 => LdAR16R8(HL, E),
            0x74 => LdAR16R8(HL, H),
            0x75 => LdAR16R8(HL, L),
            0x76 => Halt,
            0x77 => LdAR16R8(HL, A),
            0x78 => LdR8R8(A, B),
            0x79 => LdR8R8(A, C),
            0x7A => LdR8R8(A, D),
            0x7B => LdR8R8(A, E),
            0x7C => LdR8R8(A, H),
            0x7D => LdR8R8(A, L),
            0x7E => LdR8AR16(A, HL),
            0x7F => LdR8R8(A, A),
            0x80 => AddR8R8(A, B),
            0x81 => AddR8R8(A, C),
            0x82 => AddR8R8(A, D),
            0x83 => AddR8R8(A, E),
            0x84 => AddR8R8(A, H),
            0x85 => AddR8R8(A, L),
            0x86 => AddR8AR16(A, HL),
            0x87 => AddR8R8(A, A),
            0x88 => AdcR8R8(A, B),
            0x89 => AdcR8R8(A, C),
            0x8A => AdcR8R8(A, D),
            0x8B => AdcR8R8(A, E),
            0x8C => AdcR8R8(A, H),
            0x8D => AdcR8R8(A, L),
            0x8E => AdcR8AR16(A, HL),
            0x8F => AdcR8R8(A, A),
            0x90 => SubR8R8(A, B),
            0x91 => SubR8R8(A, C),
            0x92 => SubR8R8(A, D),
            0x93 => SubR8R8(A, E),
            0x94 => SubR8R8(A, H),
            0x95 => SubR8R8(A, L),
            0x96 => SubR8AR16(A, HL),
            0x97 => SubR8R8(A, A),
            0x98 => SbcR8R8(A, B),
            0x99 => SbcR8R8(A, C),
            0x9A => SbcR8R8(A, D),
            0x9B => SbcR8R8(A, E),
            0x9C => SbcR8R8(A, H),
            0x9D => SbcR8R8(A, L),
            0x9E => SbcR8AR16(A, HL),
            0x9F => SbcR8R8(A, A),
            0xA0 => AndR8R8(A, B),
            0xA1 => AndR8R8(A, C),
            0xA2 => AndR8R8(A, D),
            0xA3 => AndR8R8(A, E),
            0xA4 => AndR8R8(A, H),
            0xA5 => AndR8R8(A, L),
            0xA6 => AndR8AR16(A, HL),
            0xA7 => AndR8R8(A, A),
            0xA8 => XorR8R8(A, B),
            0xA9 => XorR8R8(A, C),
            0xAA => XorR8R8(A, D),
            0xAB => XorR8R8(A, E),
            0xAC => XorR8R8(A, H),
            0xAD => XorR8R8(A, L),
            0xAE => XorR8AR16(A, HL),
            0xAF => XorR8R8(A, A),
            0xB0 => OrR8R8(A, B),
            0xB1 => OrR8R8(A, C),
            0xB2 => OrR8R8(A, D),
            0xB3 => OrR8R8(A, E),
            0xB4 => OrR8R8(A, H),
            0xB5 => OrR8R8(A, L),
            0xB6 => OrR8AR16(A, HL),
            0xB7 => OrR8R8(A, A),
            0xB8 => CpR8R8(A, B),
            0xB9 => CpR8R8(A, C),
            0xBA => CpR8R8(A, D),
            0xBB => CpR8R8(A, E),
            0xBC => CpR8R8(A, H),
            0xBD => CpR8R8(A, L),
            0xBE => CpR8AR16(A, HL),
            0xBF => CpR8R8(A, A),
            0xC0 => RetNf(BitFlag::Z),
            0xC1 => PopR16(BC),
            0xC2 => JpNfA16(BitFlag::Z, arg16),
            0xC3 => JpA16(arg16),
            0xC4 => CallNfA16(BitFlag::Z, arg16),
            0xC5 => PushR16(BC),
            0xC6 => AddR8D8(A, arg8_0),
            0xC7 => Rst(0),
            0xC8 => RetF(BitFlag::Z),
            0xC9 => Ret,
            0xCA => JpFA16(BitFlag::Z, arg16),
            0xCC => CallFA16(BitFlag::Z, arg16),
            0xCD => CallA16(arg16),
            0xCE => AdcR8D8(A, arg8_0),
            0xCF => Rst(0x0008),
            0xD0 => RetNf(BitFlag::C),
            0xD1 => PopR16(DE),
            0xD2 => JpNfA16(BitFlag::C, arg16),
            0xD3 => Nop,
            0xD4 => CallNfA16(BitFlag::C, arg16),
            0xD5 => PushR16(DE),
            0xD6 => SubR8D8(A, arg8_0),
            0xD7 => Rst(0x0010),
            0xD8 => RetF(BitFlag::C),
            0xD9 => Reti,
            0xDA => JpFA16(BitFlag::C, arg16),
            0xDB => Nop,
            0xDC => CallFA16(BitFlag::C, arg16),
            0xDD => Nop,
            0xDE => SbcR8D8(A, arg8_0),
            0xDF => Rst(0x0018),
            0xE0 => LdhA8R8(arg8_0, A),
            0xE1 => PopR16(HL),
            0xE2 => LdhAR8R8(C, A),
            0xE3 => Nop,
            0xE4 => Nop,
            0xE5 => PushR16(HL),
            0xE6 => AndR8D8(A, arg8_0),
            0xE7 => Rst(0x0020),
            0xE8 => AddR16D8(SP, arg8_0 as i8),
            0xE9 => JpAR16(HL),
            0xEA => LdA16R8(arg16, A),
            0xEB => Nop,
            0xEC => Nop,
            0xED => Nop,
            0xEE => XorR8D8(A, arg8_0),
            0xEF => Rst(0x0028),
            0xF0 => LdhR8A8(A, arg8_0),
            0xF1 => PopR16(AF),
            0xF2 => Nop,
            0xF3 => Di,
            0xF4 => Nop,
            0xF5 => PushR16(AF),
            0xF6 => OrR8D8(A, arg8_0),
            0xF7 => Rst(0x0030),
            0xF8 => LdhlR16D8(SP, arg8_0 as i8),
            0xF9 => LdR16R16(SP, HL),
            0xFA => LdR8A16(A, arg16),
            0xFB => Ei,
            0xFE => CpR8D8(A, arg8_0),
            0xFF => Rst(0x0038),
            0xCB => match arg8_0 {
                0x00 => RlcR8(B),
                0x01 => RlcR8(C),
                0x02 => RlcR8(D),
                0x03 => RlcR8(E),
                0x04 => RlcR8(H),
                0x05 => RlcR8(L),
                0x06 => RlcAR16(HL),
                0x07 => RlcR8(A),
                0x08 => RrcR8(B),
                0x09 => RrcR8(C),
                0x0A => RrcR8(D),
                0x0B => RrcR8(E),
                0x0C => RrcR8(H),
                0x0D => RrcR8(L),
                0x0E => RrcAR16(HL),
                0x0F => RrcR8(A),
                0x10 => RlR8(B),
                0x11 => RlR8(C),
                0x12 => RlR8(D),
                0x13 => RlR8(E),
                0x14 => RlR8(H),
                0x15 => RlR8(L),
                0x16 => RlAR16(HL),
                0x17 => RlR8(A),
                0x18 => RrR8(B),
                0x19 => RrR8(C),
                0x1A => RrR8(D),
                0x1B => RrR8(E),
                0x1C => RrR8(H),
                0x1D => RrR8(L),
                0x1E => RrAR16(HL),
                0x1F => RrR8(A),
                0x20 => SlaR8(B),
                0x21 => SlaR8(C),
                0x22 => SlaR8(D),
                0x23 => SlaR8(E),
                0x24 => SlaR8(H),
                0x25 => SlaR8(L),
                0x26 => SlaAR16(HL),
                0x27 => SlaR8(A),
                0x28 => SraR8(B),
                0x29 => SraR8(C),
                0x2A => SraR8(D),
                0x2B => SraR8(E),
                0x2C => SraR8(H),
                0x2D => SraR8(L),
                0x2E => SraAR16(HL),
                0x2F => SraR8(A),
                0x30 => SwapR8(B),
                0x31 => SwapR8(C),
                0x32 => SwapR8(D),
                0x33 => SwapR8(E),
                0x34 => SwapR8(H),
                0x35 => SwapR8(L),
                0x36 => SwapAR16(HL),
                0x37 => SwapR8(A),
                0x38 => SrlR8(B),
                0x39 => SrlR8(C),
                0x3A => SrlR8(D),
                0x3B => SrlR8(E),
                0x3C => SrlR8(H),
                0x3D => SrlR8(L),
                0x3E => SrlAR16(HL),
                0x3F => SrlR8(A),
                0x40 => BitR8(0, B),
                0x41 => BitR8(0, C),
                0x42 => BitR8(0, D),
                0x43 => BitR8(0, E),
                0x44 => BitR8(0, H),
                0x45 => BitR8(0, L),
                0x46 => BitAR16(0, HL),
                0x47 => BitR8(0, A),
                0x48 => BitR8(1, B),
                0x49 => BitR8(1, C),
                0x4A => BitR8(1, D),
                0x4B => BitR8(1, E),
                0x4C => BitR8(1, H),
                0x4D => BitR8(1, L),
                0x4E => BitAR16(1, HL),
                0x4F => BitR8(1, A),
                0x50 => BitR8(2, B),
                0x51 => BitR8(2, C),
                0x52 => BitR8(2, D),
                0x53 => BitR8(2, E),
                0x54 => BitR8(2, H),
                0x55 => BitR8(2, L),
                0x56 => BitAR16(2, HL),
                0x57 => BitR8(2, A),
                0x58 => BitR8(3, B),
                0x59 => BitR8(3, C),
                0x5A => BitR8(3, D),
                0x5B => BitR8(3, E),
                0x5C => BitR8(3, H),
                0x5D => BitR8(3, L),
                0x5E => BitAR16(3, HL),
                0x5F => BitR8(3, A),
                0x60 => BitR8(4, B),
                0x61 => BitR8(4, C),
                0x62 => BitR8(4, D),
                0x63 => BitR8(4, E),
                0x64 => BitR8(4, H),
                0x65 => BitR8(4, L),
                0x66 => BitAR16(4, HL),
                0x67 => BitR8(4, A),
                0x68 => BitR8(5, B),
                0x69 => BitR8(5, C),
                0x6A => BitR8(5, D),
                0x6B => BitR8(5, E),
                0x6C => BitR8(5, H),
                0x6D => BitR8(5, L),
                0x6E => BitAR16(5, HL),
                0x6F => BitR8(5, A),
                0x70 => BitR8(6, B),
                0x71 => BitR8(6, C),
                0x72 => BitR8(6, D),
                0x73 => BitR8(6, E),
                0x74 => BitR8(6, H),
                0x75 => BitR8(6, L),
                0x76 => BitAR16(6, HL),
                0x77 => BitR8(6, A),
                0x78 => BitR8(7, B),
                0x79 => BitR8(7, C),
                0x7A => BitR8(7, D),
                0x7B => BitR8(7, E),
                0x7C => BitR8(7, H),
                0x7D => BitR8(7, L),
                0x7E => BitAR16(7, HL),
                0x7F => BitR8(7, A),
                0x80 => ResR8(0, B),
                0x81 => ResR8(0, C),
                0x82 => ResR8(0, D),
                0x83 => ResR8(0, E),
                0x84 => ResR8(0, H),
                0x85 => ResR8(0, L),
                0x86 => ResAR16(0, HL),
                0x87 => ResR8(0, A),
                0x88 => ResR8(1, B),
                0x89 => ResR8(1, C),
                0x8A => ResR8(1, D),
                0x8B => ResR8(1, E),
                0x8C => ResR8(1, H),
                0x8D => ResR8(1, L),
                0x8E => ResAR16(1, HL),
                0x8F => ResR8(1, A),
                0x90 => ResR8(2, B),
                0x91 => ResR8(2, C),
                0x92 => ResR8(2, D),
                0x93 => ResR8(2, E),
                0x94 => ResR8(2, H),
                0x95 => ResR8(2, L),
                0x96 => ResAR16(2, HL),
                0x97 => ResR8(2, A),
                0x98 => ResR8(3, B),
                0x99 => ResR8(3, C),
                0x9A => ResR8(3, D),
                0x9B => ResR8(3, E),
                0x9C => ResR8(3, H),
                0x9D => ResR8(3, L),
                0x9E => ResAR16(3, HL),
                0x9F => ResR8(3, A),
                0xA0 => ResR8(4, B),
                0xA1 => ResR8(4, C),
                0xA2 => ResR8(4, D),
                0xA3 => ResR8(4, E),
                0xA4 => ResR8(4, H),
                0xA5 => ResR8(4, L),
                0xA6 => ResAR16(4, HL),
                0xA7 => ResR8(4, A),
                0xA8 => ResR8(5, B),
                0xA9 => ResR8(5, C),
                0xAA => ResR8(5, D),
                0xAB => ResR8(5, E),
                0xAC => ResR8(5, H),
                0xAD => ResR8(5, L),
                0xAE => ResAR16(5, HL),
                0xAF => ResR8(5, A),
                0xB0 => ResR8(6, B),
                0xB1 => ResR8(6, C),
                0xB2 => ResR8(6, D),
                0xB3 => ResR8(6, E),
                0xB4 => ResR8(6, H),
                0xB5 => ResR8(6, L),
                0xB6 => ResAR16(6, HL),
                0xB7 => ResR8(6, A),
                0xB8 => ResR8(7, B),
                0xB9 => ResR8(7, C),
                0xBA => ResR8(7, D),
                0xBB => ResR8(7, E),
                0xBC => ResR8(7, H),
                0xBD => ResR8(7, L),
                0xBE => ResAR16(7, HL),
                0xBF => ResR8(7, A),
                0xC0 => SetR8(0, B),
                0xC1 => SetR8(0, C),
                0xC2 => SetR8(0, D),
                0xC3 => SetR8(0, E),
                0xC4 => SetR8(0, H),
                0xC5 => SetR8(0, L),
                0xC6 => SetAR16(0, HL),
                0xC7 => SetR8(0, A),
                0xC8 => SetR8(1, B),
                0xC9 => SetR8(1, C),
                0xCA => SetR8(1, D),
                0xCB => SetR8(1, E),
                0xCC => SetR8(1, H),
                0xCD => SetR8(1, L),
                0xCE => SetAR16(1, HL),
                0xCF => SetR8(1, A),
                0xD0 => SetR8(2, B),
                0xD1 => SetR8(2, C),
                0xD2 => SetR8(2, D),
                0xD3 => SetR8(2, E),
                0xD4 => SetR8(2, H),
                0xD5 => SetR8(2, L),
                0xD6 => SetAR16(2, HL),
                0xD7 => SetR8(2, A),
                0xD8 => SetR8(3, B),
                0xD9 => SetR8(3, C),
                0xDA => SetR8(3, D),
                0xDB => SetR8(3, E),
                0xDC => SetR8(3, H),
                0xDD => SetR8(3, L),
                0xDE => SetAR16(3, HL),
                0xDF => SetR8(3, A),
                0xE0 => SetR8(4, B),
                0xE1 => SetR8(4, C),
                0xE2 => SetR8(4, D),
                0xE3 => SetR8(4, E),
                0xE4 => SetR8(4, H),
                0xE5 => SetR8(4, L),
                0xE6 => SetAR16(4, HL),
                0xE7 => SetR8(4, A),
                0xE8 => SetR8(5, B),
                0xE9 => SetR8(5, C),
                0xEA => SetR8(5, D),
                0xEB => SetR8(5, E),
                0xEC => SetR8(5, H),
                0xED => SetR8(5, L),
                0xEE => SetAR16(5, HL),
                0xEF => SetR8(5, A),
                0xF0 => SetR8(6, B),
                0xF1 => SetR8(6, C),
                0xF2 => SetR8(6, D),
                0xF3 => SetR8(6, E),
                0xF4 => SetR8(6, H),
                0xF5 => SetR8(6, L),
                0xF6 => SetAR16(6, HL),
                0xF7 => SetR8(6, A),
                0xF8 => SetR8(7, B),
                0xF9 => SetR8(7, C),
                0xFA => SetR8(7, D),
                0xFB => SetR8(7, E),
                0xFC => SetR8(7, H),
                0xFD => SetR8(7, L),
                0xFE => SetAR16(7, HL),
                0xFF => SetR8(7, A),
                _ => Nop,
            },
            _ => Nop,
        }
    }
}

impl Instruction {
    pub fn get_size(self) -> u8 {
        use self::Instruction::*;
        {
            match self {
                Nop
                | Stop
                | Halt
                | AdcR8AR16(_, _)
                | AdcR8R8(_, _)
                | AddR16R16(_, _)
                | AddR8AR16(_, _)
                | AddR8R8(_, _)
                | AndR8R8(_, _)
                | AndR8AR16(_, _)
                | Ccf
                | Cpl
                | CpR8AR16(_, _)
                | CpR8R8(_, _)
                | DaaR8(_)
                | DecAR16(_)
                | DecR16(_)
                | DecR8(_)
                | Di
                | Ei
                | IncAR16(_)
                | IncR16(_)
                | IncR8(_)
                | JpAR16(_)
                | LddAR16R8(_, _)
                | LddR8AR16(_, _)
                | LdhAR8R8(_, _)
                | LdiAR16R8(_, _)
                | LdiR8AR16(_, _)
                | LdAR16R8(_, _)
                | LdR16R16(_, _)
                | LdR8R8(_, _)
                | LdR8AR16(_, _)
                | OrR8AR16(_, _)
                | OrR8R8(_, _)
                | PopR16(_)
                | PushR16(_)
                | Rlca
                | Rla
                | Rrca
                | Rra
                | SbcR8AR16(_, _)
                | SbcR8R8(_, _)
                | Scf
                | SubR8AR16(_, _)
                | SubR8R8(_, _)
                | XorR8AR16(_, _)
                | XorR8R8(_, _)
                | Ret
                | Reti
                | RetF(_)
                | RetNf(_)
                | Rst(_) => 1,

                BitAR16(_, _)
                | BitR8(_, _)
                | LdR8D8(_, _)
                | AdcR8D8(_, _)
                | AddR8D8(_, _)
                | AddR16D8(_, _)
                | AndR8D8(_, _)
                | CpR8D8(_, _)
                | JrA8(_)
                | JrFA8(_, _)
                | JrNfA8(_, _)
                | LdhlR16D8(_, _)
                | LdhA8R8(_, _)
                | LdhR8A8(_, _)
                | LdAR16D8(_, _)
                | OrR8D8(_, _)
                | ResAR16(_, _)
                | ResR8(_, _)
                | SbcR8D8(_, _)
                | SetAR16(_, _)
                | SetR8(_, _)
                | SubR8D8(_, _)
                | SwapAR16(_)
                | SwapR8(_)
                | XorR8D8(_, _)
                | RlcR8(_)
                | RlcAR16(_)
                | RlR8(_)
                | RlAR16(_)
                | RrcR8(_)
                | RrcAR16(_)
                | RrR8(_)
                | RrAR16(_)
                | SlaR8(_)
                | SlaAR16(_)
                | SraR8(_)
                | SraAR16(_)
                | SrlR8(_)
                | SrlAR16(_) => 2,
                LdR8A16(_, _)
                | JpA16(_)
                | JpFA16(_, _)
                | JpNfA16(_, _)
                | LdA16R16(_, _)
                | LdA16R8(_, _)
                | LdR16D16(_, _)
                | CallA16(_)
                | CallFA16(_, _)
                | CallNfA16(_, _) => 3,
            }
        }
    }
}
