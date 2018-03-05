use std::io;
use std::io::prelude::*;
use std::fs::File;
use shared::*;

fn load_rom_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = try!(File::open(path));
    let mut buffer: Vec<u8> = Vec::new();
    try!(f.read_to_end(&mut buffer));
    Ok(buffer)
}

pub fn load_rom(path: &str) -> Result<Box<Cartridge>, String> {
    let bytes = match load_rom_bytes(path) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };
    let header = parse_header(bytes.clone())?;
    let rom = parse_rom(header, bytes)?;
    Ok(rom)
}

pub fn print_rom(path: &str) {
    let bytes = load_rom_bytes(path).expect("Didn't load correctly");
    let header = parse_header(bytes.clone()).expect("Couldn't parse header");
    println!("header: {:?}", header);
    let blocks = split_to_blocks(bytes);
    println!("block count: {}", blocks.len());
    let rom = load_rom(path).unwrap();
    let head2 = rom.get_header();
    println!(
        "rom type: {:?}, rom color: {:?}, japanese?: {}",
        head2.model, head2.color, head2.japanese
    );
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
    Required,
}

#[derive(Debug)]
pub struct CartridgeHeader {
    title: String,
    color: ColorSupport,
    model: CartridgeType,
    logo: Vec<u8>,
    rom_size_kb: u16,
    rom_banks: u16,
    ram_size_kb: u16,
    ram_banks: u16,
    japanese: bool,
    checksum: u8,
}
type Block16Kb = [u8; 0x4000];

fn parse_cartridge_type(code: u8) -> Result<CartridgeType, String> {
    match code {
        0x00 => Ok(CartridgeType::ROM),
        0x1 | 0x2 | 0x3 => Ok(CartridgeType::MBC1),
        0x5 | 0x6 => Ok(CartridgeType::MBC2),
        0xf | 0x10 | 0x11 | 0x12 | 0x13 => Ok(CartridgeType::MBC3),
        0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e => Ok(CartridgeType::MBC5),
        _ => Err("Unknown cartridge type code".to_string()),
    }
}
fn parse_rom_size(code: u8) -> Result<(u16, u16), String> {
    match code {
        0x0 => Ok((32, 1)),
        0x1 => Ok((64, 4)),
        0x2 => Ok((128, 8)),
        0x3 => Ok((256, 16)),
        0x4 => Ok((512, 32)),
        0x5 => Ok((1024, 64)),
        0x6 => Ok((2048, 128)),
        0x7 => Ok((4096, 256)),
        0x8 => Ok((8192, 512)),
        _ => Err("Unrecognized rom size code".to_string()),
    }
}
fn parse_ram_size(code: u8) -> (u16, u16) {
    match code {
        0x1 => (2, 0),
        0x2 => (8, 0),
        0x3 => (32, 4),
        0x4 => (128, 16),
        0x5 => (64, 8),
        _ => (0, 0),
    }
}

fn parse_header(dat: Vec<u8>) -> Result<CartridgeHeader, String> {
    let title: String = dat[0x0134..0x0143]
        .iter()
        .map(|x| x.clone() as char)
        .collect();
    let color = match dat[0x0143] {
        0x80 => ColorSupport::Supported,
        0xc0 => ColorSupport::Required,
        _ => ColorSupport::None,
    };
    let logo: Vec<u8> = dat[0x104..0x0133].iter().cloned().collect();
    let model = parse_cartridge_type(dat[0x0147])?;
    let (romsize, rombanks) = parse_rom_size(dat[0x0148])?;
    let (ramsize, rambanks) = parse_ram_size(dat[0x0149]);
    let japanese = dat[0x014a] == 0x00;
    let checksum = dat[0x014d];

    return Ok(CartridgeHeader {
        title: title,
        color: color,
        model: model,
        logo: logo,
        rom_size_kb: romsize,
        rom_banks: rombanks,
        ram_size_kb: ramsize,
        ram_banks: rambanks,
        japanese: japanese,
        checksum: checksum,
    });
}

fn split_to_blocks(dat: Vec<u8>) -> Vec<Block16Kb> {
    let size = 0x4000;
    let end_address = dat.len();
    let block_count = ((end_address as f64) / (size as f64)).ceil() as usize;
    let mut res: Vec<Block16Kb> = Vec::new();
    for i in 0..block_count {
        let offset = i * size;
        if offset + size < end_address {
            let mut block = [0; 0x4000];
            block.clone_from_slice(&dat[offset..offset + size]);
            res.push(block);
        } else {
            let mut temp: Vec<u8> = dat[offset..end_address].iter().cloned().collect();
            temp.resize(size, 0);
            let mut block = [0; 0x4000];
            block.clone_from_slice(&temp[..]);
            res.push(block);
        }
    }
    return res;
}

fn parse_rom(header: CartridgeHeader, data: Vec<u8>) -> Result<Box<Cartridge>, String> {
    match header.model {
        CartridgeType::ROM => Ok(Box::new(RomCartridge {
            header: header,
            memory: split_to_blocks(data),
        })),
        _ => Err("Not supported yet".to_string()),
    }
}

// pub struct Mbc1Cartridge{
//     header : CartridgeHeader,
//     blocks : Vec<Block16Kb>,
//     current_block : usize
// }

pub struct RomCartridge {
    header: CartridgeHeader,
    memory: Vec<Block16Kb>,
}
pub trait Cartridge {
    fn get_header(&self) -> &CartridgeHeader;
    fn get_block_0(&self) -> &Block16Kb;
    fn get_block_1(&self) -> &Block16Kb;
    fn swap_block_1(&mut self, bank: usize);
    fn read8(&self, addr: u16) -> u8;
    fn read16(&self, addr: u16) -> u16;
}

impl Cartridge for RomCartridge {
    fn get_header(&self) -> &CartridgeHeader {
        &self.header
    }
    fn get_block_0(&self) -> &Block16Kb {
        &self.memory[0]
    }
    fn get_block_1(&self) -> &Block16Kb {
        &self.memory[1]
    }
    ///Does nothing, as there is no memory bank controller
    fn swap_block_1(&mut self, _bank: usize) {
        ()
    }
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0...0x3fff => self.get_block_0()[addr as usize],
            0x4000...0x7fff => self.get_block_1()[(0x4000 - addr) as usize],
            _ => 0,
        }
    }
    fn read16(&self, addr: u16) -> u16 {
        match addr {
            0x0...0x3ffe => join_u8(self.get_block_0()[addr as usize], self.get_block_0()[(addr+1) as usize]),
            0x3fff => join_u8(self.get_block_0()[0x3fff], self.get_block_1()[0x0]),
            0x4000...0x7ffe => join_u8(self.get_block_1()[(0x4000 - addr) as usize], self.get_block_1()[(0x4001 - addr) as usize]),
            //todo: THIS IS PROBABLY THE WRONG WAY TO HANDLE THIS address OVERFLOW
            0x7fff => join_u8(self.get_block_1()[0x3fff],0),
            _ => 0,
        }
    }
}


// pub struct CartridgeUnit{
//     rom : Box<Cartridge>
// }
