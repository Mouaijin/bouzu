type Addr = u16;
type imm8 = u8;
type imm16 = u16;

#[derive(Debug)]
enum Reg8 {
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
enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
enum Instruction {
    ///No-op
    NOP,  
    ///Halt until interrupt
    HALT, 
    ///Halt cpu completely
    STOP, 
    ///swap register nibbles
    SWAP_R8(Reg8), 
    ///swap address nibbles
    SWAP_R16(Reg16), 
    ///load immediate into 8-bit register
    LD_R8_D8(Reg8,imm8), 
    ///load referenced value into 8-bit register
    LD_R8_A16(Reg16, Addr), 
    /// Store value in 8 bit register into address    
    LD_A16_R8(Addr,Reg8)

}