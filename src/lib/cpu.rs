use std::error::Error;
use std::fs;
use std::io::Read;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub V: [u8; 16],
    pub I: u16,
    pub PC: u16,
    pub gdc: [u8; 64 * 32],
    pub delay_time: u8,
    pub sound_time: u8,
    pub stack: [u8; 16],
    pub sp: u8,
    pub key: [u8; 16]
}

impl Chip8{
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>>{
        let file_content = read_file(file_name)?;
        let mem = load_rom(file_content);
        Ok(Self{
            memory: mem,
            V: [0; 16],
            I: 0,
            PC: 0x200,
            gdc: [0; 64 * 32],
            delay_time: 0,
            sound_time: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16]
        })
    }
}

fn read_file(filename: &str) -> Result<[u8; 4096 - 0x200], Box<dyn Error>>{
    let mut f = fs::File::open(filename)?;
    let mut buf: [u8; 4096 - 0x200] = [0; 4096 - 0x200];
    f.read(&mut buf)?;
    Ok(buf)
}

fn load_rom(file_content: [u8; 4096 - 0x200]) -> [u8; 4096]{
    let mut mem = [0u8; 4096];
    let pc = 0x200;
    for  i in 0..4096 - 0x200{
        mem[pc + i] = file_content[i];
    }
    mem
}