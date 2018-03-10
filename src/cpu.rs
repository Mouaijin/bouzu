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
        if use_carry && self.register.flag_is_set(BitFlag::C){
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

        let carry = match use_carry && self.register.flag_is_set(BitFlag::C){
            true => 1,
            false => 0
        };
        //Set half-carry
        if nth_bit(low_nibble(imm) + low_nibble(self.register.a) + carry,3) {
            self.register.set_flag(BitFlag::H)
        }

        //store result
        self.register.a = result as u8;
    }
    ///Subtracts an immediate ubyte from the A register with optional carry
    ///Sets Z,C,N(1),H
    fn sub8(&mut self, imm : Du8, use_carry: bool){
        let mut result: i16 = imm as i16 - self.register.a as i16;

        if use_carry && self.register.flag_is_set(BitFlag::C){
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
        let carry = match use_carry && self.register.flag_is_set(BitFlag::C){
            true => 1,
            false => 0
        };
       if low_nibble(self.register.a) as i8 + low_nibble(imm) as i8  - carry < 0 {
            self.register.set_flag(BitFlag::H)
        }
        self.register.a = result as u8;
    }
}
///Instruction logic
impl Cpu{

}