use register::*;
use shared::*;
use instructions::*;
use mmu;
use std::rc::*;
use std::cell::*;
use std::boxed::Box;

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
        self.register.set_flag_b(BitFlag::C, result < 0);
        //Set zero
        self.register.set_flag_b(BitFlag::Z, result == 0);
        //Set SUB flag
        self.register.set_flag(BitFlag::N);
        let carry = (use_carry && self.register.flag_is_set(BitFlag::C)) as i8;
        self.register.set_flag_b(
            BitFlag::H,
            (low_nibble(result.clone() as u8) as i8 - low_nibble(imm) as i8 - carry) < 0,
        );
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
        let mut result: i16 = imm as i16 - self.register.a.clone() as i16;

        //Set carry
        self.register.set_flag_b(BitFlag::C, result < 0);
        //Set zero
        self.register.set_flag_b(BitFlag::Z, result == 0);
        //Set SUB flag
        self.register.set_flag(BitFlag::N);
        self.register.set_flag_b(
            BitFlag::H,
            (low_nibble(result.clone() as u8) as i8 - low_nibble(imm) as i8) < 0,
        );
    }
    ///Increases the referenced value by one
    ///Sets Z,N(0),H
    fn inc8(&mut self, byte: &mut u8) {
        let val = match byte.clone() {
            0xFF => 0,
            x => x + 1,
        };
        *byte = val;
        self.register.set_flag_b(BitFlag::Z, *byte == 0);

        if nth_bit(val, 3) {
            self.register.set_flag(BitFlag::H);
        }
        self.register.clear_flag(BitFlag::N);
    }
    ///Increases the referenced register by one
    ///Sets Z,N(0),H
    fn inc8_reg(&mut self, reg: Reg8Name) {
        let val = match self.register.get_reg8(reg.clone()) {
            0xFF => 0,
            x => x + 1,
        };
        self.register.set_reg8(reg, val);
        self.register.set_flag_b(BitFlag::Z, val == 0);

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
    ///Decreases the referenced value by one
    ///Sets Z, N(0), H
    fn dec8_reg(&mut self, reg: Reg8Name) {
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
    fn rlca(&mut self) {
        let newcarry = nth_bit(self.register.a.clone(), 7);

        self.register.set_flag_b(BitFlag::C, newcarry);
        self.register.a = self.register.a.rotate_left(1);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
    }
    ///Rotate Left Circular. This instruction rotates either register r of the byte located at the address in HL left one bit, placing bit 7 at bit 0 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rlc(&mut self, byte: &mut u8) {
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 7));
        *byte = byte.rotate_left(1);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    ///Rotate Left Circular. This instruction rotates either register r of the byte located at the address in HL left one bit, placing bit 7 at bit 0 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rlc_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone()).clone();
        let new = old.clone().rotate_left(1);
        self.register.set_reg8(reg, new);
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 7));
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Rotate Left Accumulator. This instruction rotates A left one bit, placing bit 7 into the Carry flag and the contents of the Carry flag into bit 0 of A
    /// Sets C,N(0),H(0)
    fn rla(&mut self) {
        let newcarry = nth_bit(self.register.a.clone(), 7);
        let carry: u8 = self.register.flag_is_set(BitFlag::C) as u8;
        self.register.set_flag_b(BitFlag::C, newcarry);

        self.register.a = (self.register.a << 1) | carry;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
    }
    /// Rotate Left. This instruction rotates either the byte located at the address in HL left one bit, placing bit 7 into the Carry flag and the contents of the Carry flag into bit 0 of A
    /// Sets Z,C,N(0),H(0)
    fn rl(&mut self, byte: &mut u8) {
        let carry: u8 = self.register.flag_is_set(BitFlag::C) as u8;
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 7));
        *byte = (*byte << 1) | carry;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }

    /// Rotate Left. This instruction rotates the register L left one bit, placing bit 7 into the Carry flag and the contents of the Carry flag into bit 0 of A
    /// Sets Z,C,N(0),H(0)
    fn rl_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let carry: u8 = self.register.flag_is_set(BitFlag::C) as u8;
        let new = old.clone() << 1 | carry;

        self.register
            .set_flag_b(BitFlag::C, nth_bit(old.clone(), 7));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Rotate Right Circular Accumulator. This instruction rotates A right one bit, placing bit 0 at bit 7 AND in the Carry flag.
    /// Sets C,N(0),H(0)
    fn rrca(&mut self) {
        let newcarry = nth_bit(self.register.a.clone(), 0);

        self.register.set_flag_b(BitFlag::C, newcarry);
        self.register.a = self.register.a.rotate_right(1);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
    }
    /// Rotate Right Circular. This instruction rotates the byte located at the address in HL right one bit, placing bit 0 at bit 7 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rrc(&mut self, byte: &mut u8) {
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 0));
        *byte = byte.rotate_right(1);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);

        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    /// Rotate Right Circular. This instruction rotates the register right one bit, placing bit 0 at bit 7 AND in the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn rrc_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let new = old.rotate_right(1);
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 0));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);

        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Rotate Right Accumulator. This instruction rotates A right one bit, placing bit 0 into the Carry flag and the contents of the Carry flag into bit 7 of A
    /// Sets C,N(0),H(0)
    fn rra(&mut self) {
        let newcarry = nth_bit(self.register.a.clone(), 0);
        let carry: u8 = (self.register.flag_is_set(BitFlag::C) as u8) << 7;
        self.register.set_flag_b(BitFlag::C, newcarry);

        self.register.a = (self.register.a >> 1) | carry;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
    }
    /// Rotate Right. This instruction rotates either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag and the contents of the Carry flag into bit 7 of A
    /// Sets Z,C,N(0),H(0)
    fn rr(&mut self, byte: &mut u8) {
        let carry: u8 = (self.register.flag_is_set(BitFlag::C) as u8) << 7;
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 0));
        *byte = (*byte >> 1) | carry;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    /// Rotate Right. This instruction rotates either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag and the contents of the Carry flag into bit 7 of A
    /// Sets Z,C,N(0),H(0)
    fn rr_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let carry: u8 = (self.register.flag_is_set(BitFlag::C) as u8) << 7;
        let new = old >> 1 | carry;
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 0));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Shift Left Arithmetically. This instruction shifts either register r or the byte located at the address in HL left one bit, placing 0 into bit 0, and placing bit 7 into the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn sla(&mut self, byte: &mut u8) {
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 7));
        *byte = *byte << 1;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    /// Shift Left Arithmetically. This instruction shifts either register r or the byte located at the address in HL left one bit, placing 0 into bit 0, and placing bit 7 into the Carry flag.
    /// Sets Z,C,N(0),H(0)
    fn sla_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let new = old << 1;
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 7));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Shift Right Arithmetically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag, and leaving bit 7 untouched.
    /// Sets Z,C,N(0),H(0)
    fn sra(&mut self, byte: &mut u8) {
        let mask = *byte & 0b10000000;
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 0));
        *byte = (*byte >> 1) | mask;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    /// Shift Right Arithmetically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing bit 0 into the Carry flag, and leaving bit 7 untouched.
    /// Sets Z,C,N(0),H(0)
    fn sra_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let mask = old & 0b10000000;

        let new = old >> 1 | mask;
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 0));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }
    /// Shift Right Logically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing 0 into bit 7, and placing bit 0 into the Carry flag.
    /// Sets Z,C,H(0),N(0)
    fn srl(&mut self, byte: &mut u8) {
        self.register
            .set_flag_b(BitFlag::C, nth_bit(byte.clone(), 0));
        *byte = *byte >> 1;
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, *byte == 0);
    }
    /// Shift Right Logically. This instruction shifts either register r or the byte located at the address in HL right one bit, placing 0 into bit 7, and placing bit 0 into the Carry flag.
    /// Sets Z,C,H(0),N(0)
    fn srl_reg(&mut self, reg: Reg8Name) {
        let old = self.register.get_reg8(reg.clone());
        let new = old >> 1;
        self.register.set_flag_b(BitFlag::C, nth_bit(old, 0));
        self.register.set_reg8(reg, new);
        self.register.clear_flag(BitFlag::N);
        self.register.clear_flag(BitFlag::H);
        self.register.set_flag_b(BitFlag::Z, new == 0);
    }

    ///Tests bit b in register r or the byte addressed in HL. Basically the specified bit gets copied to the Z flag AND INVERTED.
    ///Sets Z, N(0),H(1)
    fn bit(&mut self, byte: &mut u8, b: u8) {
        self.register.set_flag_b(BitFlag::Z, !nth_bit(*byte, b));
        self.register.set_flag(BitFlag::H);
        self.register.clear_flag(BitFlag::N);
    }
    ///Tests bit b in register r or the byte addressed in HL. Basically the specified bit gets copied to the Z flag AND INVERTED.
    ///Sets Z, N(0),H(1)
    fn bit_reg(&mut self, reg: Reg8Name, b: u8) {
        let old = self.register.get_reg8(reg);
        self.register.set_flag_b(BitFlag::Z, !nth_bit(old, b));
        self.register.set_flag(BitFlag::H);
        self.register.clear_flag(BitFlag::N);
    }
    ///Sets (1) bit b in register r or the byte addressed in HL.
    ///No flags
    fn set(&mut self, byte: &mut u8, b: u8) {
        *byte |= 1 << b;
    }
    ///Sets (1) bit b in register r or the byte addressed in HL.
    ///No flags
    fn set_reg(&mut self, reg: Reg8Name, b: u8) {
        let old = self.register.get_reg8(reg.clone());
        self.register.set_reg8(reg, old | 1 << b);
    }
    ///Resets (0) bit b in register r or the byte addressed in HL.
    ///No flags
    fn reset(&mut self, byte: &mut u8, b: u8) {
        let mut mask: u8 = 0b11111110;
        mask.rotate_left(b as u32);
        *byte &= mask;
    }
    ///Resets (0) bit b in register r or the byte addressed in HL.
    ///No flags
    fn reset_reg(&mut self, reg: Reg8Name, b: u8) {
        let mut mask: u8 = 0b11111110;
        mask.rotate_left(b as u32);
        let old = self.register.get_reg8(reg.clone());
        self.register.set_reg8(reg, old & mask);
    }
}
///Instruction logic
impl Cpu {
    fn inc_pc(&mut self, ins: Instruction) {}
    pub fn run_ins(&mut self, mmu: &mut mmu::Mmu, ins: Instruction) {
        use instructions::Instruction::*;
        use register::Reg8Name::*;
        use register::Reg16Name::*;
        match ins {
            Nop => (),
            Halt => (),
            Stop => (),
            SwapR8(reg) => swap8(self.register.get_reg8_ref(reg)),
            SwapAR16(reg) => {
                let v = swap16(self.register.get_reg16(reg.clone()));
                self.register.set_reg16(reg.clone(), v)
            }
            LdR8D8(reg, imm) => *self.register.get_reg8_ref(reg) = imm,
            LdR8A16(reg, addr) => *self.register.get_reg8_ref(reg) = mmu.read8(addr),
            LdA16R8(addr, reg) => mmu.write8(addr, self.register.get_reg8(reg)),
            LdR8R8(to, from) => {
                let val = self.register.get_reg8(from);
                self.register.set_reg8(to, val);
            }
            LdR16D16(reg, imm) => self.register.set_reg16(reg, imm),
            LdR16R16(to, from) => {
                let val = self.register.get_reg16(from);
                self.register.set_reg16(to, val);
            }
            LdAR16R8(add_reg, reg) => {
                let addr = self.register.get_reg16(add_reg);
                mmu.write8(addr, self.register.get_reg8(reg));
            }
            LdAR16D8(reg, imm) => mmu.write8(self.register.get_reg16(reg), imm),
            LdR8AR16(to, from) => {
                let val = mmu.read8(self.register.get_reg16(from));
                *self.register.get_reg8_ref(to) = val;
            }
            LdA16R16(to, from) => mmu.write16(to, self.register.get_reg16(from)),
            LdiAR16R8(to, from) => {
                let val = self.register.get_reg8(from);
                let addr = self.register.get_reg16(to.clone());
                mmu.write8(addr, val);
                self.register.inc_reg16(to);
            }
            LddAR16R8(to, from) => {
                let val = self.register.get_reg8(from);
                let addr = self.register.get_reg16(to.clone());
                mmu.write8(addr, val);
                self.register.dec_reg16(to);
            }
            LdiR8AR16(to, from) => {
                let addr = self.register.get_reg16(from.clone());
                let val = mmu.read8(addr);
                self.register.set_reg8(to, val);
                self.register.inc_reg16(from);
            }
            LddR8AR16(to, from) => {
                let addr = self.register.get_reg16(from.clone());
                let val = mmu.read8(addr);
                self.register.set_reg8(to, val);
                self.register.dec_reg16(from);
            }
            LdhR8A8(to, from_lo) => {
                let val = mmu.read8(0xff00 | from_lo as u16);
                self.register.set_reg8(to, val);
            }
            LdhA8R8(to_lo, from) => {
                let val = self.register.get_reg8(from);
                mmu.write8(0xff00 | to_lo as u16, val);
            }
            LdhAR8R8(to_lo_reg, from) => {
                let addr = 0xff00 | self.register.get_reg8(to_lo_reg) as u16;
                mmu.write8(addr, self.register.get_reg8(from));
            }
            //no clue if this is right
            LdhlR16D8(to, imm) => {
                let new = self.register.sp + imm as u8;
                self.register.set_reg16(to, new);
                self.register.sp = new;
            }
            IncR8(reg) => (),
            IncR16(reg) => (),
            IncAR16(reg) => (),
            DecR8(reg) => (),
            DecR16(reg) => (),
            DecAR16(reg) => (),
            Scf => self.register.set_flag(BitFlag::C),
            Ccf => self.register.clear_flag(BitFlag::C),
            BitR8(bit, reg) => (),
            BitAR16(bit, reg) => (),
            ResR8(bit, reg) => (),
            ResAR16(bit, reg) => (),
            SetR8(bit, reg) => (),
            SetAR16(bit, reg) => (),
            Cpl => {
                self.register.set_flag(BitFlag::N);
                self.register.set_flag(BitFlag::H);
                self.register.a = self.register.a ^ 0xff;
            }
            Rlca => self.rlca(),
            Rla => self.rla(),
            Rrca => self.rrca(),
            Rra => self.rra(),
            RlcR8(reg) => self.rlc_reg(reg),
            RlcAR16(reg) => {
                let addr = self.register.get_reg16(reg);
                let mut val = mmu.read8(addr.clone());
                self.rlc(&mut val);
                mmu.write8(addr, val);
            }
            RlR8(reg) => self.rl_reg(reg),
            RlAR16(reg) => {
                let addr = self.register.get_reg16(reg);
                let mut val = mmu.read8(addr.clone());
                self.rl(&mut val);
                mmu.write8(addr, val);
            }
            RrcR8(reg) => self.rrc_reg(reg),
            RrcAR16(reg) => (),
            RrR8(reg) => self.rr_reg(reg),
            RrAR16(reg) => (),
            SlaR8(reg) => self.sla_reg(reg),
            SlaAR16(reg) => (),
            SraR8(reg) => self.sra_reg(reg),
            SraAR16(reg) => (),
            SrlR8(reg) => self.srl_reg(reg),
            SrlAR16(reg) => (),
            JpA16(addr) => (),
            JpAR16(reg) => (),
            JpFA16(flag, addr) => (),
            JpNfA16(flag, addr) => (),
            JrA8(offset) => (),
            JrFA8(flag, offset) => (),
            JrNfA8(flag, offset) => (),
            AddR8R8(to, from) => (),
            AddR8D8(reg, imm) => (),
            AddR8AR16(to, from) => (),
            AddR16R16(to, from) => (),
            AddR16D8(to, imm) => (),
            AdcR8R8(to, from) => (),
            AdcR8D8(reg, imm) => (),
            AdcR8AR16(to, from) => (),
            SubR8R8(to, from) => (),
            SubR8D8(to, imm) => (),
            SubR8AR16(to, from) => (),
            SbcR8R8(to, from) => (),
            SbcR8AR16(to, from) => (),
            SbcR8D8(to, imm) => (),
            AndR8R8(to, from) => (),
            AndR8D8(to, imm) => (),
            AndR8AR16(to, from) => (),
            OrR8R8(to, from) => (),
            OrR8D8(to, imm) => (),
            OrR8AR16(to, from) => (),
            XorR8R8(to, from) => (),
            XorR8D8(to, imm) => (),
            XorR8AR16(to, from) => (),
            Ei => (),
            Di => (),
            CpR8R8(to, from) => (),
            CpR8AR16(to, from) => (),
            CpR8D8(to, imm) => (),
            DaaR8(reg) => (),
            PushR16(reg) => (),
            PopR16(reg) => (),
            CallA16(addr) => {
                self.register.sp -= 2;
                let pc = self.register.pc.clone();
                mmu.write16(self.register.sp, pc);
                self.register.pc = addr;
            }
            CallFA16(flag, addr) => (),
            CallNfA16(flag, addr) => (),
            Ret => {
                let pc = mmu.read16(self.register.sp);
                self.register.sp += 2;
                self.register.pc = pc;
            }
            Reti => (),
            RetF(flag) => (),
            RetNf(flag) => (),
            Rst(addr) => (),
        }
    }

    //CALL subtracts 2 bytes from SP after pushing return address (0xfffe) RET adds it back to SP and moves PC back to the address there
}
