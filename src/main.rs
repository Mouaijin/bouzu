mod shared;
mod instructions;
mod rom;
mod mmu;
mod register;

fn main() {
    rom::print_rom("roms/real/tetris.gb");
}
