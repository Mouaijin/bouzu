mod register;
mod rom;
mod mmu;

fn main() {
    rom::print_rom("roms/tetris.gb");
}
