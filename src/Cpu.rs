use super::sdl2;
use super::screen;
use super::keyboard::Keyboard;

use std::io::Read;
use super::rand::{thread_rng, Rng};
use std::fs::File;
use std::error::Error;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const CHIP8_FONTSET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0,
  0x20, 0x60, 0x20, 0x20, 0x70,
  0xF0, 0x10, 0xF0, 0x80, 0xF0,
  0xF0, 0x10, 0xF0, 0x10, 0xF0,
  0x90, 0x90, 0xF0, 0x10, 0x10,
  0xF0, 0x80, 0xF0, 0x10, 0xF0,
  0xF0, 0x80, 0xF0, 0x90, 0xF0,
  0xF0, 0x10, 0x20, 0x40, 0x40,
  0xF0, 0x90, 0xF0, 0x90, 0xF0,
  0xF0, 0x90, 0xF0, 0x10, 0xF0,
  0xF0, 0x90, 0xF0, 0x90, 0x90,
  0xE0, 0x90, 0xE0, 0x90, 0xE0,
  0xF0, 0x80, 0x80, 0x80, 0xF0,
  0xE0, 0x90, 0x90, 0x90, 0xE0, 
  0xF0, 0x80, 0xF0, 0x80, 0xF0,
  0xF0, 0x80, 0xF0, 0x80, 0x80
];

pub struct Cpu {

        memory: Vec<u8>,
    	opcode: u16,
	    vreg: Box<[u8; 16]>,
	    ireg: u16,
	    pc: usize,
	    graphics: Box<[u8; WIDTH/8 * HEIGHT/8]>,
	    delay: u8,
	    sound: u8,
	    stack: Box<[u16; 16]>,
	    sp: usize,
	    screen: screen::screen,
	    keys: Keyboard,
}

impl Cpu{

    pub fn new() -> Cpu{

        let sdl_context = sdl2::init().unwrap();
        			
       	Cpu{
        	memory: vec![0; 4096],
        	opcode: 0,
        	vreg: Box::new([0; 16]),
        	ireg: 0,
        	pc: 0,
			graphics: Box::new([0; WIDTH/8 * HEIGHT/8]),
        	delay: 0,
        	sound: 0,
        	stack: Box::new([0; 16]),
        	sp: 0,
        	screen: screen::screen::new(&sdl_context),
        	keys: Keyboard::new(&sdl_context),
       	}
    }

    pub fn init(&mut self) {

       	self.pc = 0x200;

       	for i in 0..80{
       		self.memory[i] = CHIP8_FONTSET[i];
       	}

        		
   	}

	pub fn read(&mut self, path: &str){

    	let mut file: File = match File::open(&path) {
        	Err(why) => panic!("couldn't open file because {} ", why.description()),
       		Ok(file) => file,
   		};
		let i = 80;
    	for byte in file.bytes() {
       		self.memory[i] = byte.unwrap();
    	}
    			
	}
		
   	pub fn cycle(&mut self) {

		let op = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc+1]) as u16;

		self.opcode = op as u16;

		match self.opcode & 0xF000 {

			0x0000 => {

    			match self.opcode & 0x00FF {

        			0x00E0 => {
           				self.clear();
           				self.pc += 2;
        			},
        			0x00EE => {
           				self.pc = self.stack[self.sp] as usize;
           				self.sp -= 1;
       				},
       				_ => {},
    			}
			},
			0x1000 => {
    			self.pc = (self.opcode & 0x0FFF) as usize;
			},
			0x2000 => {
    			self.sp += 1;
    			self.stack[self.sp] = self.pc as u16;
    			self.pc = (self.opcode & 0x0FFF) as usize;
			},
			0x3000 => {
    			let vx = ((self.opcode & 0x0F00) >> 8) as usize;
    			if self.vreg[vx] == (self.opcode & 0x00FF) as u8{
        			self.pc += 2;
    			}
    			self.pc += 2;
    		},
			0x4000 => {
				let vx = ((self.opcode & 0x0F00) >> 8) as usize;
				if self.vreg[vx] != (self.opcode & 0x00FF) as u8 {
    				self.pc += 2;
				}
				self.pc += 2;
			},
			0x5000 => {
    			let vx = (self.opcode & 0x0F00 >> 8) as usize;
    			let vy = (self.opcode & 0x00F0 >> 4) as usize;
    			if self.vreg[vx] == self.vreg[vy] {
					self.pc += 2;
   				}
    			self.pc += 2;
			},
			0x6000 => {
				self.vreg[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
				self.pc += 2;
			},
			0x7000 => {
				self.vreg[((self.opcode & 0x0F00) >> 8) as usize] += (self.opcode & 0x00FF) as u8;
				self.pc += 2;
			},
			0x8000 => {
	   			let x = (self.opcode & 0x0F00 >> 8) as usize;
    			let y = (self.opcode & 0x00F0 >> 4) as usize;
    			match self.opcode & 0x000F {

        			0x0000 => {self.vreg[x] = self.vreg[y]},

       				0x0001 => {self.vreg[x] = self.vreg[y] | self.vreg[x];},

       				0x0002 => {self.vreg[x] = self.vreg[y] & self.vreg[x];},

       				0x0003 => {self.vreg[x] = self.vreg[y] ^ self.vreg[x];},

       				0x0004 => {
           				
          				let sum = self.vreg[x] as u16 + self.vreg[y] as u16;
           				if sum > 255 {
               					self.vreg[0xF] = 1;
           				}else{
               					self.vreg[0xF] = 0;
            				}
            				self.vreg[x] = sum as u8;
            				
            			},

        			0x0005 => {

           				if self.vreg[x] > self.vreg[y] {
               					self.vreg[0xF] = 1;
            				}else{
               					self.vreg[0xF] = 0;
            				}
            				self.vreg[x] = self.vreg[x] - self.vreg[y];

           			},

        			0x0006 => {
            			
            				if (self.vreg[x] & 0b1) == 1 {
                				self.vreg[0xF] = 1;
            				}else{
                				self.vreg[0xF] = 0;
            				}
            				self.vreg[x] = self.vreg[x] >> 1;
            				
            			},

        			0x0007 => {
            				if self.vreg[y] > self.vreg[x] {
                				self.vreg[0xF] = 1;
            				}else{
                				self.vreg[0xF] = 0;
            				}
            				self.vreg[x] = self.vreg[y] - self.vreg[x];
           			},

        			0x000E => {
            				let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            				let val = self.vreg[vx];
            				if val & 0x80 > 0 {
               					self.vreg[0xF] = 1;
            				}else {
               					self.vreg[0xF] = 0;
           				}
            				self.vreg[vx] = self.vreg[vx] << 1;
           			},

        			_ => {},

				}
				self.pc +=2;

			},

			0x9000 => {
				if self.vreg[((self.opcode & 0x0F00) >> 8) as usize] != self.vreg[((self.opcode & 0x00F0) >> 4) as usize] {
					self.pc += 2;
				}
				self.pc += 2;
			},

   			0xA000 => {
				self.ireg = (self.opcode & 0x0FFF) as u16;
				self.pc += 2;
   			},

    			0xB000 => {
        			self.pc = (self.opcode & 0x00000FFF + self.vreg[0] as u16) as usize;
    			},
    		0xC000 => {
        		// CxKK
        		// Vx = rnd AND KK
        		let mut rng = thread_rng();
        		let vx = (self.opcode & 0x0F00 >> 8) as usize;
        		let rand = rng.gen_range(0, 255);
        		self.vreg[vx] = ((rand) & (self.opcode & 0x00FF)) as u8;
    		},
   		0xD000 => {
       		
       			//TO DO
        		// Dxyn
        		// Display n byte sprite
        		// Reads n bytes from memory, starting at address in I
        		// These bytes are displayed on screet at address (Vx, Vy)
        		// XOR bytes of memory into display bytes and if any display bytes
        		// are erased, VF is set to 1
        		// Wrap display around to other side of screen
        		//
        		let n = self.opcode & 0x000F;
        		let vx = self.opcode & 0x0F00 >> 8;
        		let vy = self.opcode & 0x00F0 >> 4;
        		for i in 0..n {
       				let x = self.vreg[vx as usize] as u16 + i as u16;
           			let y = self.vreg[vy as usize] as u16 + i as u16;
            			
					let memory = self.memory[(self.ireg + i) as usize];
					if memory & self.graphics[i as usize] > 0 {
    						self.vreg[0xF] = 1;
					}
					
        		}
    		},
    		0xE000 => {
        		let x = (self.opcode & 0x0F00) >> 8;
               		// Ex?? 
               		// instructions that operate off of keys
               		// _________________    _________________
               		// | 1 | 2 | 3 | C |	| 1 | 2 | 3 | 4 |
               		// | 4 | 5 | 6 | D | =	| Q | W | E | R |
               		// | 7 | 8 | 9 | E |	| A | S | D | F |
               		// | A | 0 | B | F |	| Z | X | C | V |
          			// _________________	_________________
               		// 
            				
        		match self.opcode & 0x00FF {
            		0x009E => {
               			// 9E - SKP Vx
                		// Skip next instruction if key with the value of Vx is pressed
                		//
                	},
            		0x00A1 => {
                		// A1 - SKPN Vx
                		// Skip next instruction if key with teh value of Vx is not pressed
            		},
            		_ => {},
        		};
    		},

    		0xF000 => {

        		let vx = (self.opcode & 0x0F00) >> 8;
        		match self.opcode & 0x00FF {
           			0x0007 => {self.vreg[vx as usize] = self.delay;},
           			0x000A => {},
           			0x0015 => {self.delay = self.vreg[vx as usize];},
           			0x0018 => {self.sound = self.vreg[vx as usize];},
           			0x001E => {self.ireg = self.ireg + self.vreg[vx as usize] as u16;},
           			0x0029 => {},
           			0x0033 => {
               			self.memory[self.ireg as usize] = self.vreg[vx as usize] / 100;
               			self.memory[self.ireg as usize + 1] = (self.vreg[vx as usize] / 10)%10;
                		self.memory[self.ireg as usize +2] = self.vreg[vx as usize] % 10;
                		},
            			0x0055 => {
                			for x in 0..16 {
                    			self.memory[(self.ireg + x) as usize] = self.vreg[x as usize];
                			}
                		},
            			0x0065 => {
                			for x in 0..16 {
                    			self.vreg[x as usize] = self.memory[self.ireg as usize + x as usize];
                			}
                   		},
                   		_ => {},
        		}
    		},
    		_ => { println!("Improper Instruction") },
    			
	}

       	if self.delay > 0 {

            self.delay -= 1;

        }

       	if self.sound > 0 {

			if self.sound == 1 {
    			println!("BEEP!");
			}
			self.sound -= 1;
       	}
       	
        self.screen.draw_array(&self.graphics);
	}

    fn clear(&mut self) {

       	for i in 0..(8 * 4) {
           	self.graphics[i] = 0;
       	} 		
	}
}
