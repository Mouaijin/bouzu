use std::io;
use std::io::prelude::*;
use std::fs::File;

fn load_rom_bytes(path : &str) -> Result<Vec<u8>, io::Error>{
    let mut f = try!(File::open(path));
    let mut buffer : Vec<u8> = Vec::new(); 
    try!(f.read_to_end(&mut buffer));
    Ok(buffer)
}

pub fn print_rom(path : &str){
    let bytes = load_rom_bytes(path).expect("Didn't load correctly");
    print!("{:?}", bytes);
}

#[derive(Debug)]
enum CartridgeType {
    MBC1,
    MBC2,
    MBC3,
    MBC4,
    MBC5,
}

#[derive(Debug)]
enum ColorSupport {
    None,
    Supported,
    Required
}

#[derive(Debug)]
struct CartridgeHeader {
    title : String,
    color: ColorSupport,
    model : CartridgeType,
    rom_size_kb : u16,
    rom_banks : u8,
    ram_size_kb : u16,
    ram_banks : u8,
    japanese : bool,
    checksum : u8
}

struct Cartridge{

}