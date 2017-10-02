use std::thread::sleep;
use std::time::Duration;

use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2;

const DELAY: u64 = 2;

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


pub struct Keyboard {
    event_pump: sdl2::EventPump,
    pub keys: [bool; 16],
    last_pressed: u8,
    used: bool,
    pub close: bool,
}

impl Keyboard {
    
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        
        Keyboard {
            event_pump: sdl_context.event_pump().unwrap(),
            keys: [false; 16],
            last_pressed: 0,
            used: false,
            close: false,
        }
        
    }

    pub fn handle_press(&mut self) -> u8{

		let events: Vec<Event> = self.event_pump.poll_iter().collect();
		
        for event in events {
            match event {
                Event::Quit { .. } => {
                    self.close = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {self.key_pressed(Keys::key_1 as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {self.key_pressed(Keys::key_2 as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {self.key_pressed(Keys::key_3 as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {self.key_pressed(Keys::key_4 as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {self.key_pressed(Keys::key_Q as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {self.key_pressed(Keys::key_W as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {self.key_pressed(Keys::key_E as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {self.key_pressed(Keys::key_R as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {self.key_pressed(Keys::key_A as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {self.key_pressed(Keys::key_S as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {self.key_pressed(Keys::key_D as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {self.key_pressed(Keys::key_F as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {self.key_pressed(Keys::key_Z as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {self.key_pressed(Keys::key_X as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {self.key_pressed(Keys::key_C as u8, true)},
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {self.key_pressed(Keys::key_V as u8, true)},
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {self.key_pressed(Keys::key_1 as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {self.key_pressed(Keys::key_2 as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {self.key_pressed(Keys::key_3 as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {self.key_pressed(Keys::key_4 as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {self.key_pressed(Keys::key_Q as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {self.key_pressed(Keys::key_W as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {self.key_pressed(Keys::key_E as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {self.key_pressed(Keys::key_R as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {self.key_pressed(Keys::key_A as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {self.key_pressed(Keys::key_S as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {self.key_pressed(Keys::key_D as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::F), .. } => {self.key_pressed(Keys::key_F as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {self.key_pressed(Keys::key_Z as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {self.key_pressed(Keys::key_X as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::C), .. } => {self.key_pressed(Keys::key_C as u8, false)},
                Event::KeyUp { keycode: Some(Keycode::V), .. } => {self.key_pressed(Keys::key_V as u8, false)},
                _ =>{return 1;},
            }
        }
        0
    }

	pub fn wait(&mut self) -> u8 {

    	self.used = false;

    	loop {
        	
        	self.handle_press();
        	
        	if self.used {
            	break;
        	}

        	sleep(Duration::from_millis(DELAY));
        	
    	}
    	
    	self.last_pressed
	}
	
    fn key_pressed(&mut self, key: u8, pressed: bool){
        self.keys[key as usize] = pressed;
        self.last_pressed = key;
        self.used = true;
    }
}
