use std::error::Error;
use std::fs;
use std::io::Read;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub V: [u8; 16],  // registers
    pub I: u16,  //index register
    pub pc: u16,  // program counter
    pub gdc: [u8; 64 * 32],  // screen
    pub delay_time: u8,
    pub sound_time: u8,
    pub stack: [u16; 16],
    pub sp: u8,  //stack pointer
    pub key: [bool; 16]
}

impl Chip8{
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>>{
        let file_content = read_file(file_name)?;
        let mem = load_rom(file_content);
        Ok(Self{
            memory: mem,
            V: [0; 16],
            I: 0,
            pc: 0x200,
            gdc: [0; 64 * 32],
            delay_time: 0,
            sound_time: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16]
        })
    }

    pub fn next_op(&mut self){
        self.pc += 2;
    }

    pub fn set_address(&mut self, adress: u16){
        if adress < 0x200 || adress > 0xfff{
            panic!("set program counter address - {} overflow  memory", adress);
        }
        self.pc = adress;
    }

    pub fn push_to_stack(&mut self, pos: u16){
        if self.sp > 16 {
            panic!("push to stack over flow!")
        }
        self.stack[self.sp as usize] = pos;
        self.sp += 1;
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        self.sp -= 1;
        if self.sp < 0{
            panic!("pop from stack overflow stack is empty!")
        }
        let pos = self.stack[self.sp as usize];
        self.stack[self.sp as usize] = 0;
        return pos
    }

    pub fn set_register(&mut self, v_pointer: u8, val: u8) {
        self.V[v_pointer as usize] = val;
    }

    pub fn get_register(&mut self, v_pointer: u8) -> u8
    {
        self.V[v_pointer as usize]
    }

    pub fn get_index(&mut self) -> u16{ self.I }

    pub fn set_index(&mut self, val: u16) {
        self.I = val;
    }
    pub fn get_key(&mut self, key_pointer: u8) -> bool {
        self.key[key_pointer as usize]
    }
    pub fn get_delay(&mut self) -> u8 { self.delay_time }

    pub fn set_delay(&mut self, val: u8) { self.delay_time = val }

    pub fn set_sound(&mut self, val: u8) { self.sound_time = val }

    pub fn set_memory(&mut self, pos: u16, val: u8){
        self.memory[pos as usize] = val
    }

    pub fn read_memory(&mut self, pos: u16) -> u8{
        self.memory[pos as usize]
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

