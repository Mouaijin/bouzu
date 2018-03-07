use register::*;
use shared::*;
use instructions::*;
use mmu;

pub struct Cpu {
    ///CPU register
    register: CpuRegister,
}
impl Cpu {
    //ALU logic
    ///Adds an immediate ubyte to the A register with optional carry
    /// Sets Z,C,N(0),H
    fn add8(&mut self, imm: Du8, carry: bool) {
        let mut result: u16 = imm as u16 + self.register.a as u16;
        if carry {
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

        //Set half-carry
        if nth_bit(imm, 3) && nth_bit(self.register.a, 3) {
            self.register.set_flag(BitFlag::H)
        }

        //store result
        self.register.a = result as u8;
    }
}
