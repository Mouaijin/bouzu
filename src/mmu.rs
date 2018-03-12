use rom;
use register;
use shared::*;

pub struct Mmu {
    ///cartridge provides 0x0000 - 0x7fff in two banks
    rom: Box<rom::Cartridge>,
    ///0x8000 - 0x9fff (1fff wide)
    vram: [u8; 0x1fff],
    ///0xc000 - 0xcfff (1fff wide) (0xc000 - 0xddff echoed in 0xe000-0xfdff)
    work_ram_0: [u8; 0x1fff],
    ///0xd000 - 0xdfff (1fff wide) (1 bank in DMG, 1~7 in CGB) (0xc000 - 0xddff echoed in 0xe000-0xfdff)
    work_ram_1: [u8; 0x1fff],
    ///0xfe00 - 0xfe9f (0x9f wide)
    sprite_table: [u8; 0x9f],
    ///0xff00 - 0xff7f (0x7f wide)
    ///todo: encapsulate IO memory for easier use
    io_registers: [u8; 0x7f],
    ///0xff80 - 0xfffe (0x7e wide)
    hram: [u8; 0x7e],
    /// 0xffff
    interrupts: u8,
}

impl Mmu {
    pub fn new(rom: Box<rom::Cartridge>) -> Self {
        Mmu {
            rom: rom,
            vram: [0; 0x1fff],
            work_ram_0: [0; 0x1fff],
            work_ram_1: [0; 0x1fff],
            sprite_table: [0; 0x9f],
            io_registers: [0; 0x7f],
            hram: [0; 0x7e],
            interrupts: 0,
        }
    }
    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            //rom memory banks
            0x0000...0x7fff => self.rom.read8(addr),
            0x8000...0x9fff => self.vram[0x8000 - addr as usize],
            //external ram (handled by cartridge)
            0xa000...0xbfff => self.rom.read8(addr),
            //work ram 0
            0xc000...0xcfff => self.work_ram_0[0xa000 - addr as usize],
            //work ram 1..n
            0xd000...0xdfff => self.work_ram_1[0xd000 - addr as usize],
            //echo ram
            0xe000...0xfdff => self.read8(addr - 0x2000),
            //sprite table
            0xfe00...0xfe9f => self.sprite_table[0xfe00 - addr as usize],
            //unusable, I'll just return a 0
            0xfea0...0xfeff => 0,
            //io registers
            0xff00...0xff7f => self.io_registers[0xff00 - addr as usize],
            //hram
            0xff80...0xfffe => self.hram[0xff80 - addr as usize],
            //interrupt register
            0xffff => self.interrupts,
            _ => 0,
        }
    }
    // pub fn ref8(&self, addr: u16) -> u8 {
    //     match addr {
    //         //rom memory banks
    //         0x0000...0x7fff => self.rom.read8(addr),
    //         0x8000...0x9fff => self.vram[0x8000 - addr as usize],
    //         //external ram (handled by cartridge)
    //         0xa000...0xbfff => self.rom.read8(addr),
    //         //work ram 0
    //         0xc000...0xcfff => self.work_ram_0[0xa000 - addr as usize],
    //         //work ram 1..n
    //         0xd000...0xdfff => self.work_ram_1[0xd000 - addr as usize],
    //         //echo ram
    //         0xe000...0xfdff => self.read8(addr - 0x2000),
    //         //sprite table
    //         0xfe00...0xfe9f => self.sprite_table[0xfe00 - addr as usize],
    //         //unusable, I'll just return a 0
    //         0xfea0...0xfeff => 0,
    //         //io registers
    //         0xff00...0xff7f => self.io_registers[0xff00 - addr as usize],
    //         //hram
    //         0xff80...0xfffe => self.hram[0xff80 - addr as usize],
    //         //interrupt register
    //         0xffff => self.interrupts,
    //         _ => 0,
    //     }
    // }
    pub fn read16(&self, addr: u16) -> u16 {
        if addr < 0xffff {
            join_u8(self.read8(addr), self.read8(addr + 1))
        } else {
            //todo: probably wrong
            //this just left-shifts the last address's value 8 places when reading past the last byte
            join_u8(self.read8(addr), 0)
        }
    }
    pub fn write8(&mut self, addr: Addr, dat: u8) {
        match addr {
            //rom memory banks
            0x8000...0x9fff => self.vram[0x8000 - addr as usize] = dat,
            //external ram (handled by cartridge)
            // 0xa000...0xbfff => self.rom.read8(addr),
            //work ram 0
            0xc000...0xcfff => self.work_ram_0[0xa000 - addr as usize] = dat,
            //work ram 1..n
            0xd000...0xdfff => self.work_ram_1[0xd000 - addr as usize] = dat,
            //echo ram
            0xe000...0xfdff => self.work_ram_0[addr as usize - 0x2000] = dat,
            //sprite table
            0xfe00...0xfe9f => self.sprite_table[0xfe00 - addr as usize] = dat,
            //unusable, I'll just return a 0
            // 0xfea0...0xfeff => 0,
            //io registers
            0xff00...0xff7f => self.io_registers[0xff00 - addr as usize] = dat,
            //hram
            0xff80...0xfffe => self.hram[0xff80 - addr as usize] = dat,
            //interrupt register
            0xffff => self.interrupts = dat,
            _ => (),
        }
    }
    pub fn write16(&mut self, addr: Addr, dat: u16) {
        let (hi, lo) = split_u16(dat);
        self.write8(addr, hi);
        self.write8(addr + 1, lo);
    }
}
