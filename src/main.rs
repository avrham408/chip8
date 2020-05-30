use chip8::{cpu, fetch_and_decode};
use std::process;

fn main() {
    println!("set up graphic");
    println!("set up input");

    let cpu = cpu::Chip8::new("chip_games/PONG").unwrap_or_else(|e|{
        println!("Loading chip8 file error : {}", e);
        process::exit(1);
    }); // #TODO add support for reading args from std::env:args
    //loop {} game loop
}
