#[derive(Debug)]
pub enum Reg8Name {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}
#[derive(Debug)]
pub enum Reg16Name {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
pub enum BitFlag {
    ///Zero flag (This bit becomes set (1) if the result of an operation has been zero (0). Used for conditional jumps. )
    Z,
    ///(DAA) Indicates whether the previous instruction has been an addition or subtraction
    N,
    ///(DAA) Indicates carry for lower 4bits of the result
    H,
    ///Carry flag (Becomes set when the result of an addition became bigger than FFh (8bit) or FFFFh (16bit), Or when the result of a subtraction or comparision became less than zero)
    ///Also the flag becomes set when a rotate/shift operation has shifted-out a "1"-bit. Used for conditional jumps, and for instructions such like ADC, SBC, RL, RLA, etc. 
    C,
}

pub struct CpuRegister {
    ///Accumulator
    a: u8,
    ///Flags
    ///```markdown
    ///      Bit  Name  Set Clr  Expl.
    ///  7    zf    Z   NZ   Zero Flag
    ///  6    n     -   -    Add/Sub-Flag (BCD)
    ///  5    h     -   -    Half Carry Flag (BCD)
    ///  4    cy    C   NC   Carry Flag
    ///  3-0  -     -   -    Not used (always zero)
    ///```
    f: u8,
    ///BC high
    b: u8,
    ///BC low
    c: u8,
    ///DE high
    d: u8,
    ///DE low
    e: u8,
    ///HL high
    h: u8,
    ///HL low
    l: u8,
    ///Stack pointer
    sp: u16,
    ///Program Counter/Pointer
    pc: u16,
}

impl CpuRegister{
    pub fn new() -> Self{
        CpuRegister{a:0,f:0,b:0,c:0,d:0,e:0,h:0,l:0,sp:0,pc:0}
    }
    pub fn set_flag(&mut self, flag: BitFlag){
        match flag {
            BitFlag::Z => self.f |= 0b00000001,
            BitFlag::N => self.f |= 0b00000010,
            BitFlag::H => self.f |= 0b00000100,
            BitFlag::C => self.f |= 0b00001000
        }
    }
    pub fn clear_flag(&mut self, flag: BitFlag){
        match flag {
            BitFlag::Z => self.f &= 0b11111110,
            BitFlag::N => self.f &= 0b11111101,
            BitFlag::H => self.f &= 0b11111011,
            BitFlag::C => self.f &= 0b11110111
        }
    }
    pub fn flag_is_set(&self, flag: BitFlag) -> bool{
        match flag{
            BitFlag::Z => self.f & 0b00000001 == 1,
            BitFlag::N => (self.f & 0b00000010) >> 1 == 1,
            BitFlag::H => (self.f & 0b00000100) >> 2 == 1,
            BitFlag::C => (self.f & 0b00001000) >> 3 == 1
        }
    }
    pub fn flag_is_unset(&self, flag: BitFlag) -> bool{
        !self.flag_is_set(flag)
    }
    pub fn set_reg8(&mut self, reg : Reg8Name, val : u8){
        match reg{
            Reg8Name::A => self.a = val,
            Reg8Name::B => self.b = val,
            Reg8Name::C => self.c = val,
            Reg8Name::D => self.d = val,
            Reg8Name::E => self.e = val,
            Reg8Name::F => self.f = val,
            Reg8Name::H => self.h = val,
            Reg8Name::L => self.l = val,
        }
    }
    fn set_reg8_pair(&mut self, hi : Reg8Name, lo : Reg8Name, val: u16){
        let dat = split_u16(val);
        self.set_reg8(hi, dat.0);
        self.set_reg8(lo, dat.1);
    }
    pub fn set_reg16(&mut self, reg : Reg16Name, val : u16){
        match reg{
            Reg16Name::AF => self.set_reg8_pair(Reg8Name::A, Reg8Name::F, val),
            Reg16Name::BC => self.set_reg8_pair(Reg8Name::B, Reg8Name::C, val),
            Reg16Name::DE => self.set_reg8_pair(Reg8Name::A, Reg8Name::F, val),
            Reg16Name::HL => self.set_reg8_pair(Reg8Name::A, Reg8Name::F, val),
            Reg16Name::SP => self.sp = val,
            Reg16Name::PC => self.pc = val,

        }
    }

    pub fn get_reg16(&self, reg : Reg16Name) -> u16{
        match reg{
            Reg16Name::AF => join_u8(self.a, self.f),
            Reg16Name::BC => join_u8(self.b, self.c),
            Reg16Name::DE => join_u8(self.d, self.e),
            Reg16Name::HL => join_u8(self.h, self.l),
            Reg16Name::SP => self.sp,
            Reg16Name::PC => self.pc,

        }
    }
}

fn split_u16(val : u16) -> (u8,u8){
    let hi = (val >> 8) as u8;
    let lo = ((val << 8) >> 8) as u8;
    (hi, lo)
}
fn join_u8(hi : u8, lo : u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}
