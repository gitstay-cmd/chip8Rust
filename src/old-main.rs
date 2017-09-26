extern crate sdl2;
extern crate piston;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use piston::event_loop::*;
use piston::input::*;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashSet;

enum Keys {
    key_0 = 0,
    key_1,
    key_2,
    key_3,
    key_4,
    key_5,
    key_6,
    key_7,
    key_8,
    key_9,
    key_a,
    key_b,
    key_c,
    key_d,
    key_e,
    key_f,
}

struct Cpu {
    renderer: render::Renderer<'static>,
    ireg: u16,
}

impl Cpu {

    fn draw(&mut self, x: i32, y: i32){
        self.put(x, y);
        self.present();
    }

    fn present(&mut self){
        self.renderer.present();
    }
    
    fn put(&mut self, x: i32, y: i32){
        self.renderer.draw_point(Point::new(x, y));
    }

    fn draw_array(&mut self, arrays: &[[i32; 5]; 4]){

	for x in 0..4 {
    		for y in 0..5{
        		if(arrays[x][y] == 1){
            			self.put(y as i32, x as i32);
            			println!("{}, {}", x, y);
        		}
    		}
	}
	self.present();
            
    }
}

fn editing(bor: &mut [u8]){
	for i in 0..bor.len(){
    		bor[i] = 12;
	}
}

fn print_key(key_x: Keys){
    println!("Key: {}", key_x as i32);
}


fn main() {

    	let sdl_context = sdl2::init().unwrap();

    	let mut event_pump = sdl_context.event_pump().unwrap();
    	
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Help", 640, 320)
		.position_centered()
		.build()
		.unwrap();

	let mut renderer = window.renderer().build().unwrap();

	renderer.set_scale(10.0, 10.0);

	renderer.set_draw_color(Color::RGB(0, 0, 0));
	renderer.clear();
	renderer.present();

	renderer.set_draw_color(Color::RGB(255, 255, 255));
	renderer.draw_point(Point::new(10, 10));

	renderer.present();

	let mut cpu: Cpu = Cpu{renderer: renderer, ireg: 0};
	cpu.draw(12, 22);
	
	let mut g = [0; 16];

	editing(&mut g);
	println!("{:?}", g);
	let b = Keys::key_b;

	let x: [[i32; 5]; 4] = [[1, 0, 0, 0, 1],
				[0, 1, 0, 1, 0],
				[0, 0, 1, 0, 0],
				[0, 0, 0, 0, 0]];

	cpu.draw_array(&x);
	'trust: loop {

    		for event in event_pump.poll_iter(){
                	match event {
                    		Event::Quit { .. } => {
                        		break 'trust;
                    		},
                    		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'trust; },
                    		Event::KeyDown { keycode: Some(Keycode::Num0), .. } => { print_key(Keys::key_0);},
                    		Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { print_key(Keys::key_1);},
                    		Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { print_key(Keys::key_2);},
                  		Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { print_key(Keys::key_3);},
                    		Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { print_key(Keys::key_4);},
                    		Event::KeyDown { keycode: Some(Keycode::Num5), .. } => { print_key(Keys::key_5);},
                    		Event::KeyDown { keycode: Some(Keycode::Num6), .. } => { print_key(Keys::key_6);},
                    		Event::KeyDown { keycode: Some(Keycode::Num7), .. } => { print_key(Keys::key_7);},
                    		Event::KeyDown { keycode: Some(Keycode::Num8), .. } => { print_key(Keys::key_8);},
                    		Event::KeyDown { keycode: Some(Keycode::Num9), .. } => { print_key(Keys::key_9);},
                    		Event::KeyDown { keycode: Some(Keycode::A), .. } => { print_key(Keys::key_a)},
                    		Event::KeyDown { keycode: Some(Keycode::B), .. } => { print_key(Keys::key_b)},
                    		Event::KeyDown { keycode: Some(Keycode::C), .. } => { print_key(Keys::key_c)},
                    		Event::KeyDown { keycode: Some(Keycode::D), .. } => { print_key(Keys::key_d)},
                    		Event::KeyDown { keycode: Some(Keycode::E), .. } => { print_key(Keys::key_e)},
                    		Event::KeyDown { keycode: Some(Keycode::F), .. } => { print_key(Keys::key_f)},
                            	Event::KeyUp { keycode: Some(Keycode::Num0), .. } => { print_key(Keys::key_0);},
				Event::KeyUp { keycode: Some(Keycode::Num1), .. } => { print_key(Keys::key_1);},
				Event::KeyUp { keycode: Some(Keycode::Num2), .. } => { print_key(Keys::key_2);},
				Event::KeyUp { keycode: Some(Keycode::Num3), .. } => { print_key(Keys::key_3);},
				Event::KeyUp { keycode: Some(Keycode::Num4), .. } => { print_key(Keys::key_4);},
                            	Event::KeyUp { keycode: Some(Keycode::Num5), .. } => { print_key(Keys::key_5);},
				Event::KeyUp { keycode: Some(Keycode::Num6), .. } => { print_key(Keys::key_6);},
				Event::KeyUp { keycode: Some(Keycode::Num7), .. } => { print_key(Keys::key_7);},
				Event::KeyUp { keycode: Some(Keycode::Num8), .. } => { print_key(Keys::key_8);},
				Event::KeyUp { keycode: Some(Keycode::Num9), .. } => { print_key(Keys::key_9);},
				Event::KeyUp { keycode: Some(Keycode::A), .. } => { print_key(Keys::key_a)},
				Event::KeyUp { keycode: Some(Keycode::B), .. } => { print_key(Keys::key_b)},
				Event::KeyUp { keycode: Some(Keycode::C), .. } => { print_key(Keys::key_c)},
				Event::KeyUp { keycode: Some(Keycode::D), .. } => { print_key(Keys::key_d)},
				Event::KeyUp { keycode: Some(Keycode::E), .. } => { print_key(Keys::key_e)},
				Event::KeyUp { keycode: Some(Keycode::F), .. } => { print_key(Keys::key_f)},
                    		_ => {},
                	}
    		}
    		
    		sleep(Duration::from_millis(100));
	}

}
