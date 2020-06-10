use chip8::{cpu, fetch_and_decode, execute};
use std::process;

fn main() {
    println!("set up graphic");
    println!("set up input");

    let mut cpu = cpu::Chip8::new("chip_games/PONG").unwrap_or_else(|e|{
        println!("Loading chip8 file error : {}", e);
        process::exit(1);
    }); // #TODO add support for reading args from std::env:args


    //loop {} game loop
    loop {
        let program_counter = cpu.pc as usize;
        let code: [u8; 2] = [cpu.memory[program_counter],
            cpu.memory[program_counter + 1]];
        println!("{:?}", fetch_and_decode::fetch(&code));
        let opcode = fetch_and_decode::fetch(&code);
        execute::exe(opcode, &code, &mut cpu);
    }
}
