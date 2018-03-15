mod shared;
mod instructions;
mod rom;
mod cpu;
mod mmu;
mod register;
#[macro_use]
extern crate log;

fn main() {
    // rom::print_rom("roms/real/tetris.gb");
    let rom = rom::load_rom("roms/real/tetris.gb").expect("Couldn't load rom");
    let mut mmu = mmu::Mmu::new(rom);
    let mut cpu = cpu::Cpu::new();
    for x in 0..10 {
        cpu.step(&mut mmu);
    }
}
