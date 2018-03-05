use register;
type Addr = u16;
type Du8 = u8;
type Ds8 = i8;
type BitIndex = u8;
type Du16 = u16;



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
enum Instruction {
    ///No-op
    Nop,
    ///Halt until interrupt
    Halt,
    ///Halt cpu completely
    Stop,
    ///swap register nibbles
    SwapR8(register::Reg8Name),
    ///swap address nibbles
    SwapR16(register::Reg16Name),
    ///load immediate into 8-bit register
    LdR8D8(register::Reg8Name, Du8),
    ///load referenced value into 8-bit register
    LdR8A16(register::Reg16Name, Addr),
    /// Store value in 8 bit register into address
    LdA16R8(Addr, register::Reg8Name),
    ///Load value in 8 bit register to another register
    LdR8R8(register::Reg8Name, register::Reg8Name),
    ///Load 16 bit value into 16 bit register
    LdR16D16(register::Reg16Name, Du16),
    ///Load value in 16 bit register in 16 bit register
    LdR16R16(register::Reg16Name, register::Reg16Name),
    ///Store value in 8 bit register into adress in 16 bit register
    LdAR16R8(register::Reg16Name, register::Reg8Name),
    ///Store 8 bit value in address in 16 bit register
    LdAR16D8(register::Reg16Name, Du8),
    ///Load value in address in 16 bit register into 8 bit register
    LdR8AR16(register::Reg8Name, register::Reg16Name),
    ///Store value in 16 bit register into address
    LdA16R16(Addr, register::Reg16Name),
    ///Store value in 8 bit register into address in 16 bit register and then increment 16 bit register
    LdiAR16R8(register::Reg16Name, register::Reg8Name),
    ///Store value in 8 bit register into address in 16 bit register and then decrement 16 bit register
    LddAR16R8(register::Reg16Name, register::Reg8Name),
    ///Load value in address in 16 register into 8 bit register and increment 16 bit register
    LdiR8AR16(register::Reg8Name, register::Reg16Name),
    ///Load value in address in 16 register into 8 bit register and decrement 16 bit register
    LddR8AR16(register::Reg8Name, register::Reg16Name),
    ///Load value in address (FF00 + 8 bit address) in 8 bit register
    LdhR8A8(register::Reg8Name, Du8),
    ///Store value in 8 bit register in address (FF00 + 8 bit address)
    LdhA8R8(Du8, register::Reg8Name),
    ///Store value in 8 bit register in address (FF00 + 8 bit register)
    LdhAR8R8(register::Reg8Name, register::Reg8Name),
    ///Add signed 8 bit value to SP and copy SP to 16 bit register
    LdhlR16D8(register::Reg16Name, Ds8),
    ///Inc value in 8 bit register
    IncR8(register::Reg8Name),
    ///Inc value in 16 bit register
    IncR16(register::Reg16Name),
    ///Inc value address in 16 bit register
    IncAR16(register::Reg16Name),
    ///Dec value in 8 bit register
    DecR8(register::Reg8Name),
    ///Dec value in 16 bit register
    DecR16(register::Reg16Name),
    ///Dec value address in 16 bit register
    DecAR16(register::Reg16Name),
    ///Set carry flag
    Scf,
    ///Clear carry flag
    Ccf,
    ///Test bit n in 8 bit register
    BitR8(BitIndex, register::Reg8Name),
    ///Test bit n in address in 16 bit register
    BitAR16(BitIndex, register::Reg16Name),
    ///Clear bit n in 8 bit register
    ResR8(BitIndex, register::Reg8Name),
    ///Clear bit n in address in 16 bit register
    ResAR16(BitIndex, register::Reg16Name),
    ///Set bit n in 8 bit register
    SetR8(BitIndex, register::Reg8Name),
    ///Set bit n in address in 16 bit register
    SetAR16(BitIndex, register::Reg16Name),
    ///Bitwise NOT on register A
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
    RlcR8(register::Reg8Name),
    ///Rotate value pointed by 16 bit register left with carry
    RlcAR16(register::Reg16Name),
    ///Rotate 8 bit register left
    RlR8(register::Reg8Name),
    ///Rotate value pointed by 16 bit register left
    RlAR16(register::Reg16Name),
    ///Rotate 8 bit register right with carry
    RrcR8(register::Reg8Name),
    ///Rotate value pointed by 16 bit register right with carry
    RrcAR16(register::Reg16Name),
    ///Rotate 8 bit register right
    RrR8(register::Reg8Name),
    ///Rotate value pointed by 16 bit register right
    RrAR16(register::Reg16Name),
    ///Shift 8 bit register left, preserving sign
    SlaR8(register::Reg8Name),
    ///Shift value pointed by 16 bit register left, preserving sign
    SlaAR16(register::Reg16Name),
    ///Shift 8 bit register right, preserving sign
    SraR8(register::Reg8Name),
    ///Shift value pointed by 16 bit register right, preserving sign
    SraAR16(register::Reg16Name),
    ///Shift 8 bit register right
    SrlR8(register::Reg8Name),
    ///Shift value pointed by 16 bit register right
    SrlAR16(register::Reg16Name),
    ///Absolute jump to address
    JpA16(Addr),
    ///Jump to address in address in 16 bit register (erhh)
    JpAR16(register::Reg16Name),
    ///Jump to address if flag is set
    JpFA16(register::BitFlag, Addr),
    ///Jump to address if flag is not set
    JpNfA16(register::BitFlag, Addr),
    ///Relative jump with signed offset
    JrA8(Ds8),
    ///Relative jump with signed offset if flag is set
    JrFA8(register::BitFlag, Ds8),
    ///Relative jump with signed offset if flag is not set
    JrNfA8(register::BitFlag, Ds8),
    ///Add 8 bit register to 8 bit register
    AddR8R8(register::Reg8Name, register::Reg8Name),
    ///Add 8 bit value to 8 bit register
    AddR8D8(register::Reg8Name, Du8),
    ///Add value pointed by 16 bit register to 8 bit register
    AddR8AR16(register::Reg8Name, register::Reg16Name),
    ///Add 16 bit register to 16 bit register
    AddR16R16(register::Reg16Name, register::Reg16Name),
    ///Add signed 8 bit value to 16 bit register
    AddR16D8(register::Reg16Name, Ds8),
    ///Add 8 bit register to 8 bit register with carry
    AdcR8R8(register::Reg8Name, register::Reg8Name),
    ///Add 8 bit value to 8 bit register with carry
    AdcR8D8(register::Reg8Name, Du8),
    ///Add value in address in 16 bit register to 8 bit register with carry
    AdcR8AR16(register::Reg8Name, register::Reg16Name),
    ///Subtract value in 8 bit register with 8 bit register
    SubR8R8(register::Reg8Name, register::Reg8Name),
    ///Subtract 8 bit value from 8 bit register
    SubR8D8(register::Reg8Name, Du8),
    ///Subtract value in address in 16 bit register from 8 bit register
    SubR8AR16(register::Reg8Name, register::Reg16Name),
    ///Subtract value in 8 bit register + carry from 8 bit register
    SbcR8R8(register::Reg8Name, register::Reg8Name),
    ///Subtract value in address in 16 bit register + carry from 8 bit register
    SbcR8AR16(register::Reg8Name, register::Reg16Name),
    ///Subtract 8 bit value + carry from 8 bit register
    SbcR8D8(register::Reg8Name, Du8),
    ///Bitwise AND between 8 bit registers
    AndR8R8(register::Reg8Name, register::Reg8Name),
    ///Bitwise AND between 8 bit register and 8 bit value
    AndR8D8(register::Reg8Name, Du8),
    ///Bitwise AND between 8 bit register and value in address in 16 bit register
    AndR8AR16(register::Reg8Name, register::Reg16Name),
    ///Bitwise OR between 8 bit registers
    OrR8R8(register::Reg8Name, register::Reg8Name),
    ///Bitwise OR between 8 bit register and 8 bit value
    OrR8D8(register::Reg8Name, Du8),
    ///Bitwise OR between 8 bit register and value in address in 16 bit register
    OrR8AR16(register::Reg8Name, register::Reg16Name),
    ///Bitwise XOR between 8 bit registers
    XorR8R8(register::Reg8Name, register::Reg8Name),
    ///Bitwise XOR between 8 bit register and 8 bit value
    XorR8D8(register::Reg8Name, Du8),
    ///Bitwise XOR between 8 bit register and value in address in 16 bit register
    XorR8AR16(register::Reg8Name, register::Reg16Name),
    ///Enabled interrupts
    Ei,
    ///Disable interrupts
    Di,
    ///Compare 8 bit register with 8 bit register
    CpR8R8(register::Reg8Name, register::Reg8Name),
    ///Compare 8 bit register with value in address in 16 bit register
    CpR8AR16(register::Reg8Name, register::Reg16Name),
    ///Compare 8 bit register with 8 bit value
    CpR8D8(register::Reg8Name, Du8),
    ///Converts 8 bit register into packed BCD
    DaaR8(register::Reg8Name),
    ///Push 16 bit register onto stack
    PushR16(register::Reg16Name),
    ///Pop 16 bit value from stack into 16 bit register
    PopR16(register::Reg16Name),
    ///Call routine at address
    CallA16(Addr),
    ///Call routine at address if flag is set
    CallFA16(register::BitFlag, Addr),
    ///Call routine at address if flag is not set
    CallNfA16(register::BitFlag, Addr),
    ///Return from subroutine
    Ret,
    ///Return from subroutine and enable interrupts
    Reti,
    ///Return if flag is set
    RetF(register::BitFlag),
    ///Return if flag is not set
    RetNf(register::BitFlag),
    ///Restart at address (simple call to address)
    Rst(Addr)
}
