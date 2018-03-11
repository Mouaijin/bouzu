use register::*;
use shared::*;
use instructions::*;
use mmu;

pub struct Cpu {
    ///CPU register
    register: CpuRegister,
}
///ALU logic
impl Cpu {
    ///Adds an immediate ubyte to the A register with optional carry
    /// Sets Z,C,N(0),H
    fn add8(&mut self, imm: Du8, use_carry: bool) {
        let mut result: u16 = imm as u16 + self.register.a as u16;
        if use_carry && self.register.flag_is_set(BitFlag::C) {
            result += 1;
        }
        //Set carry
        if result > 0xff {
            self.register.set_flag(BitFlag::C);
        }
        //Set zero
        if result == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        //Set ADD flag
        self.register.clear_flag(BitFlag::N);

        let carry = match use_carry && self.register.flag_is_set(BitFlag::C) {
            true => 1,
            false => 0,
        };
        //Set half-carry
        if nth_bit(low_nibble(imm) + low_nibble(self.register.a) + carry, 3) {
            self.register.set_flag(BitFlag::H)
        }

        //store result
        self.register.a = result as u8;
    }
    ///Subtracts an immediate ubyte from the A register with optional carry
    ///Sets Z,C,N(1),H
    fn sub8(&mut self, imm: Du8, use_carry: bool) {
        let mut result: i16 = imm as i16 - self.register.a as i16;

        if use_carry && self.register.flag_is_set(BitFlag::C) {
            result -= 1;
        }
        //Set carry
        if result < 0 {
            self.register.set_flag(BitFlag::C);
        }
        //Set zero
        if result == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        //Set SUB flag
        self.register.set_flag(BitFlag::N);
        let carry = match use_carry && self.register.flag_is_set(BitFlag::C) {
            true => 1,
            false => 0,
        };
        if low_nibble(self.register.a) as i8 - low_nibble(imm) as i8 - carry < 0 {
            self.register.set_flag(BitFlag::H)
        }
        self.register.a = result as u8;
    }
    ///Logical AND with A register
    ///Sets Z,C(0),N(0),H(1)
    fn and8(&mut self, imm: Du8) {
        self.register.a &= imm;
        if self.register.a == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        self.register.set_flag(BitFlag::H);
        self.register.clear_flag(BitFlag::C);
        self.register.clear_flag(BitFlag::N);
    }
    ///Logical OR with A register
    ///Sets Z, C(0), N(0), H(0)
    fn or8(&mut self, imm: Du8) {
        self.register.a |= imm;
        if self.register.a == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        self.register.clear_flag(BitFlag::H);
        self.register.clear_flag(BitFlag::C);
        self.register.clear_flag(BitFlag::N);
    }
    ///Logical XOR with A register
    ///Sets Z, C(0), N(0), H(0)
    fn xor8(&mut self, imm: Du8) {
        self.register.a ^= imm;
        if self.register.a == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        self.register.clear_flag(BitFlag::H);
        self.register.clear_flag(BitFlag::C);
        self.register.clear_flag(BitFlag::N);
    }
    ///Compares operand with A register by subtracting from A register
    ///Does not change A register, just flags
    ///Sets Z,C,N(1),H
    fn cp8(&mut self, imm: Du8) {
        let mut result: i16 = imm as i16 - self.register.a as i16;

        //Set carry
        if result < 0 {
            self.register.set_flag(BitFlag::C);
        }
        //Set zero
        if result == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        //Set SUB flag
        self.register.set_flag(BitFlag::N);

        if (low_nibble(self.register.a) as i8 - low_nibble(imm) as i8) < 0 {
            self.register.set_flag(BitFlag::H)
        }
    }
    ///Increases the referenced value by one
    ///Sets Z,N(0),H
    fn inc8(&mut self, byte : &mut u8) {
        let val = match byte.clone() {
            0xFF => 0,
            x => x + 1,
        };
        *byte = val;
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }


    ///Decreases the referenced value by one
    ///Sets Z, N(0), H
    fn dec8(&mut self, byte: &mut u8) {
        let val = match byte.clone() {
            0 => 0xFF,
            x => x - 1,
        };
        *byte = val;
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }


    ///This instruction adds the contents of the given register pair to register pair HL
    ///Sets C, N(0)
    fn add16(&mut self, reg: Reg16Name) {
        let val = self.register.get_reg16(reg.clone()) as u32
            + self.register.get_reg16(Reg16Name::HL) as u32;
        if val > 0xffff {
            self.register.set_flag(BitFlag::C)
        }
        self.register.clear_flag(BitFlag::N);
        self.register.set_reg16(Reg16Name::HL, val as u16);
    }
    ///Increments the value of the given register pair
    ///Sets {}
    fn inc16(&mut self, reg: Reg16Name) {
        let val = match self.register.get_reg16(reg.clone()) {
            0xffff => 0,
            x => x + 1,
        };
        self.register.set_reg16(reg, val);
    }
    ///Decreases the value of the given register pair
    ///Sets {}
    fn dec16(&mut self, reg: Reg16Name) {
        let val = match self.register.get_reg16(reg.clone()) {
            0 => 0xffff,
            x => x - 1,
        };
        self.register.set_reg16(reg, val);
    }
    ///Rotate Left Circular Accumulator. This instruction rotates A left one bit, placing bit 7 at bit 0 AND in the Carry flag.
    ///Sets: C, N(0),H(0)
    fn rlca(&mut self){}
    ///Rotate Left Circular. This instruction rotates either register r of the byte located at the address in HL left one bit, placing bit 7 at bit 0 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rlc(&mut self, byte : &mut u8){}
    /// Rotate Left Accumulator. This instruction rotates A left one bit, placing bit 7 into the Carry flag and the contents of the Carry flag into bit 0 of A
    /// Sets C,N(0),H(0)
    fn rla(&mut self){}
    /// Rotate Left. This instruction rotates either register r or the byte located at the address in HL left one bit, placing bit 7 into the Carry flag and the contents of the Carry flag into bit 0 of A
    /// Sets Z,C,N(0),H(0)
    fn rl(&mut self, byte : &mut u8){}
    /// Rotate Right Circular Accumulator. This instruction rotates A right one bit, placing bit 0 at bit 7 AND in the Carry flag.
    /// Sets C,N(0),H(0)
    fn rrca(&mut self){}
    /// Rotate Right Circular. This instruction rotates either register r of the byte located at the address in HL right one bit, placing bit 0 at bit 7 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rrc(&mut self, byte : &mut u8){}
    /// Rotate Right Accumulator. This instruction rotates A right one bit, placing bit 0 into the Carry flag and the contents of the Carry flag into bit 7 of A
    /// Sets C,N(0),H(0)
    fn rra(&mut self){}
    /// Rotate Right. This instruction rotates either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag and the contents of the Carry flag into bit 7 of A
    /// Sets Z,C,N(0),H(0)
    fn rr(&mut self, byte : &mut u8){}
    /// Shift Left Arithmetically. This instruction shifts either register r or the byte located at the address in HL left one bit, placing 0 into bit 0, and placing bit 7 into the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn sla(&mut self, byte : &mut u8){}
    /// Shift Right Arithmetically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag, and leaving bit 7 untouched.
    /// Sets Z,C,N(0),H(0)
    fn sra(&mut self, byte : &mut u8){}
    /// Shift Right Logically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing 0 into bit 7, and placing bit 0 into the Carry flag.
    /// Sets Z,C,H(0),N(0)
    fn srl(&mut self, byte : &mut u8){}


}
///Instruction logic
impl Cpu {}
