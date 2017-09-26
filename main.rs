extern crate sdl2;
extern crate rand;

use std::env;

mod Cpu;
mod screen;

fn main() {

    let args: Vec<String> = env::args().collect();

    let path = args[1]; 
    let mut cpu: Cpu::Cpu = Cpu::Cpu::new();
    cpu.init();
    cpu.read(&path);

    loop {
        cpu.cycle();
    }
}
