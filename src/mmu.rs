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
    pub fn read8(&self, add: u16) -> u8 {
        let addr = add as usize;
        match addr {
            //rom memory banks
            0x0000...0x7fff => self.rom.read8(add),
            0x8000...0x9fff => self.vram[addr - 0x8000],
            //external ram (handled by cartridge)
            0xa000...0xbfff => self.rom.read8(add),
            //work ram 0
            0xc000...0xcfff => self.work_ram_0[addr - 0xa000],
            //work ram 1..n
            0xd000...0xdfff => self.work_ram_1[addr - 0xd000],
            //echo ram
            0xe000...0xfdff => self.read8(add - 0x2000),
            //sprite table
            0xfe00...0xfe9f => self.sprite_table[addr - 0xfe00],
            //unusable, I'll just return a 0
            0xfea0...0xfeff => 0,
            //io registers
            0xff00...0xff7f => self.io_registers[addr - 0xff00],
            //hram
            0xff80...0xfffe => self.hram[(addr - 0xff81) as usize],
            //interrupt register
            0xffff => self.interrupts,
            _ => 0,
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        if addr < 0xffff {
            join_u8(self.read8(addr), self.read8(addr + 1))
        } else {
            //todo: probably wrong
            //this just left-shifts the last address's value 8 places when reading past the last byte
            join_u8(self.read8(addr), 0)
        }
    }
    pub fn write8(&mut self, add: Addr, dat: u8) {
        let addr = add as usize;
        match addr {
            //rom memory banks
            0x8000...0x9fff => self.vram[addr - 0x8000] = dat,
            //external ram (handled by cartridge)
            // 0xa000...0xbfff => self.rom.read8(addr),
            //work ram 0
            0xc000...0xcfff => self.work_ram_0[addr - 0xa000] = dat,
            //work ram 1..n
            0xd000...0xdfff => self.work_ram_1[addr - 0xd000] = dat,
            //echo ram
            0xe000...0xfdff => self.work_ram_0[addr - 0x2000] = dat,
            //sprite table
            0xfe00...0xfe9f => self.sprite_table[addr - 0xfe00] = dat,
            //unusable, I'll just return a 0
            // 0xfea0...0xfeff => 0,
            //io registers
            0xff00...0xff7f => self.io_registers[addr] = dat,
            //hram
            0xff80...0xfffe => self.hram[addr - 0xff81] = dat,
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

    pub fn push_stack(&mut self, sp: &mut u16, val: u16) {
        *sp -= 2;
        self.write16(*sp, val);
    }
    pub fn pop_stack(&mut self, sp: &mut u16) -> u16 {
        let val = self.read16(*sp);
        *sp += 2;
        val
    }

    pub fn enable_interrupts(&mut self) {}
    pub fn disable_interrupts(&mut self) {}
}
