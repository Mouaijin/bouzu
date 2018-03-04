use rom;
struct Mmu{
    rom : Box<rom::Cartridge>,
    ///0x8000 - 0x09fff (1fff wide)
    vram : Vec<u8>,
    ///0xc000 - 0xcfff (1fff wide) (0xc000 - 0xddff echoed in 0xe000-0xfdff)
    work_ram_0 : Vec<u8>,
    ///0xd000 - 0xdfff (1fff wide) (1 bank in DMG, 1~7 in CGB) (0xc000 - 0xddff echoed in 0xe000-0xfdff)
    work_ram_1 : Vec<u8>,
    ///0xfe00 - 0xfe9f (0x9f wide)
    sprite_table : Vec<u8>,
    ///0xff00 - 0xff7f (0x7f wide)
    io_registers : Vec<u8>,
    ///0xff80 - 0xfffe (0x7e wide)
    hram : Vec<u8>,
    /// 0xffff
    interrupts : u8

}