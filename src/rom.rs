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
    let header = parse_header(bytes.clone()).expect("Couldn't parse header");
    println!("header: {:?}", header);
    let blocks = split_to_blocks(bytes);
    println!("block count: {}", blocks.len());

}

#[derive(Debug)]
enum CartridgeType {
    ROM,
    MBC1,
    MBC2,
    MBC3,
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
    logo : Vec<u8>,
    rom_size_kb : u16,
    rom_banks : u16,
    ram_size_kb : u16,
    ram_banks : u16,
    japanese : bool,
    checksum : u8
}

fn parse_cartridge_type(code : u8) -> Result<CartridgeType,String>{
    match code{
        0x00 => Ok(CartridgeType::ROM),
        0x1 | 0x2 | 0x3 => Ok(CartridgeType::MBC1),
        0x5 | 0x6 => Ok(CartridgeType::MBC2),
        0xf | 0x10 | 0x11 | 0x12 | 0x13 => Ok(CartridgeType::MBC3),
        0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e => Ok(CartridgeType::MBC5),
        _ => Err("Unknown cartridge type code".to_string())

    }
}
fn parse_rom_size(code : u8) -> Result<(u16, u16),String>{
    match code{
        0x0 => Ok((32, 1)),
        0x1 => Ok((64, 4)),
        0x2 => Ok((128,8)),
        0x3 => Ok((256, 16)),
        0x4 => Ok((512, 32)),
        0x5 => Ok((1024, 64)),
        0x6 => Ok((2048, 128)),
        0x7 => Ok((4096,256)),
        0x8 => Ok((8192, 512)),
        _ => Err("Unrecognized rom size code".to_string())

    }
}
fn parse_ram_size(code:u8) -> (u16,u16){
    match code{
        0x1 => (2,0),
        0x2 => (8,0),
        0x3 => (32,4),
        0x4 => (128,16),
        0x5 => (64, 8),
        _ => (0,0)
    }
}

fn parse_header(dat : Vec<u8>) -> Result<CartridgeHeader, String>{
    let title : String = dat[0x0134..0x0143].iter().map(|x| x.clone() as char).collect();
    let color = match dat[0x0143]{
        0x80 => ColorSupport::Supported,
        0xc0 => ColorSupport::Required,
        _ => ColorSupport::None
    };
    let logo : Vec<u8> = dat[0x104..0x0133].iter().cloned().collect();
    let model = parse_cartridge_type(dat[0x0147])?;
    let (romsize, rombanks) = parse_rom_size(dat[0x0148])?;
    let (ramsize, rambanks) = parse_ram_size(dat[0x0149]);
    let japanese = dat[0x014a] == 0x00;
    let checksum = dat[0x014d];

    return Ok(CartridgeHeader{title : title, color: color, model : model, logo : logo, rom_size_kb : romsize, rom_banks : rombanks, ram_size_kb : ramsize, ram_banks: rambanks, japanese: japanese, checksum : checksum });
}

type Block16Kb = Vec<u8>;

pub fn split_to_blocks(dat : Vec<u8>) -> Vec<Block16Kb>{
    let size = 0x4000;
    let end_address = dat.len();
    let block_count = ((end_address as f64)/ (size as f64)).ceil() as usize;
    let mut res : Vec<Block16Kb> = Vec::new();
    for i in 0..block_count {
        let offset = i * size;
        if offset + size < end_address{
            res.push( dat[offset .. offset + size].iter().cloned().collect());
        }
        else {
            let mut temp : Vec<u8> = dat[offset .. end_address].iter().cloned().collect();
            temp.resize(size, 0);
            res.push(temp);
        }
    }
    return res;
}

struct Cartridge{
    header : CartridgeHeader,
    blocks : Vec<Block16Kb>
}