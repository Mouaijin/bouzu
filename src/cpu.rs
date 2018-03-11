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
    ///Increases the value in a register by one
    ///Sets Z,N(0),H
    fn inc8(&mut self, reg: Reg8Name) {
        let val = match self.register.get_reg8(reg.clone()) {
            0xFF => 0,
            x => x + 1,
        };
        self.register.set_reg8(reg, val);
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }
    ///Increases the value in memory by one (address in HL)
    ///Sets Z,N(0),H
    fn inc8_mem(&mut self, mut mmu: mmu::Mmu) {
        let val = match mmu.read8(self.register.get_reg16(Reg16Name::HL)) {
            0xFF => 0,
            x => x + 1,
        };
        mmu.write8(self.register.get_reg16(Reg16Name::HL), val);
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }
    ///Decreases the value in a register by one
    ///Sets Z, N(0), H
    fn dec8(&mut self, reg: Reg8Name) {
        let val = match self.register.get_reg8(reg.clone()) {
            0 => 0xFF,
            x => x - 1,
        };
        self.register.set_reg8(reg, val);
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }

    ///Decreases the value in memory by one (address in HL)
    ///Sets Z, N(0), H
    fn dec8_mem(&mut self, mut mmu: mmu::Mmu) {
        let val = match mmu.read8(self.register.get_reg16(Reg16Name::HL)) {
            0 => 0xFF,
            x => x - 1,
        };
        mmu.write8(self.register.get_reg16(Reg16Name::HL), val);
        if val == 0 {
            self.register.set_flag(BitFlag::Z);
        }
        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }
}
///Instruction logic
impl Cpu {}
