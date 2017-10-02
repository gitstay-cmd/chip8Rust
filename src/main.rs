extern crate sdl2;
extern crate rand;

use std::env;

mod Cpu;
mod screen;
mod keyboard;

fn main() {

    let args: Vec<String> = env::args().collect();

    let ref path = args[1];
    let mut cpu: Cpu::Cpu = Cpu::Cpu::new();
    cpu.init();
    cpu.read(&path);

    loop {
        cpu.cycle();
    }
}
