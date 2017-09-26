use std::thread::sleep;
use std::time::Duration;

use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2;

enum Keys {
    key_1 = 0,
    key_2,
    key_3,
    key_4,
    key_Q,
    key_W,
    key_E,
    key_R,
    key_A,
    key_S,
    key_D,
    key_F,
    key_Z,
    key_X,
    key_C,
    key_V,
}

const DELAY: u64 = 2;

pub struct keyboard {
    event_pump: sdl2::EventPump,
    pub keys: [bool; 16],
    last_pressed: u8,
    used: bool,
    pub close: bool,
}

impl Keyboard {
    
    pub fn new(sdl_context: &sdl2::Sdl) -> keyboard {
        
        keyboard {
            event_pump: sdl_context.event_pump().unwrap(),
            keys: [false; 16],
            last_pressed: 0,
            used: false,
            close: false;
        }
        
    }

    pub fn handle_press(&mut self) -> u8{
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.close = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {key_pressed(key_1, true)},
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {key_pressed(key_2, true)},
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {key_pressed(key_3, true)},
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {key_pressed(key_4, true)},
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {key_pressed(key_Q, true)},
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {key_pressed(key_W, true)},
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {key_pressed(key_E, true)},
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {key_pressed(key_R, true)},
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {key_pressed(key_A, true)},
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {key_pressed(key_S, true)},
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {key_pressed(key_D, true)},
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {key_pressed(key_F, true)},
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {key_pressed(key_Z, true)},
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {key_pressed(key_X, true)},
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {key_pressed(key_C, true)},
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {key_pressed(key_V, true)},
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {key_pressed(key_1, false)},
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {key_pressed(key_2, false)},
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {key_pressed(key_3, false)},
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {key_pressed(key_4, false)},
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {key_pressed(key_Q, false)},
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {key_pressed(key_W, false)},
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {key_pressed(key_E, false)},
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {key_pressed(key_R, false)},
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {key_pressed(key_A, false)},
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {key_pressed(key_S, false)},
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {key_pressed(key_D, false)},
                Event::KeyUp { keycode: Some(Keycode::F), .. } => {key_pressed(key_F, false)},
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {key_pressed(key_Z, false)},
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {key_pressed(key_X, false)},
                Event::KeyUp { keycode: Some(Keycode::C), .. } => {key_pressed(key_C, false)},
                Event::KeyUp { keycode: Some(Keycode::V), .. } => {key_pressed(key_V, false)},
                _ =>{},
            }
        }
    }

	pub fn wait(&mut self) -> u8{
    	self.used = false;

    	loop {
        	
        	self.handle_press();
        	
        	if(self.used){
            	break;
        	}

        	sleep(Duration::from_millis(DELAY)):
        	
    	}
    	
    	self.last_pressed;
	}
	
    fn key_pressed(&mut self, key: Keys, pressed: bool){
        self.keys[key as usize] = pressed;
        self.last_pressed = key as u8;
        self.used = true;
    }
}
