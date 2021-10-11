mod chip8;
mod cpu;
mod display;
mod input;
mod instruction;
mod rom;

use crate::chip8::Chip8;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut chip8 = Chip8::init();
    chip8.load(filename);
    chip8.start();

    Ok(())
}
