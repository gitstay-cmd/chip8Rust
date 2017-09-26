use super::sdl2::pixels::Color;
use super::sdl2;
use super::sdl2::render;
use super::sdl2::rect::Point;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct screen {
    renderer: render::Renderer<'static>,

}

impl screen {

    pub fn new(sdl_context: &sdl2::Sdl) -> screen {

	let video_subsystem = sdl_context.video().unwrap();
	
	let window = video_subsystem.window("CHIP8RUST", WIDTH as u32 * 10, HEIGHT as u32 * 10)
		.position_centered()
		.build()
		.unwrap();
	let mut renderer = window.renderer().build().unwrap();
	renderer.set_scale(10.0, 10.0);

	renderer.set_draw_color(Color::RGB(0, 0, 0));
	renderer.clear();
	renderer.present();

	renderer.set_draw_color(Color::RGB(255, 255, 255));

        screen {
            renderer: renderer,
	}
	
    }

    pub fn draw(&mut self, x: i32, y: i32){
        self.put(x, y);
        self.present();
    }

    fn present(&mut self){
        self.renderer.present();
    }

    fn put(&mut self, x: i32, y: i32){
        self.renderer.draw_point(Point::new(x, y));
    }

    pub fn draw_array(&mut self, arrays: &Box<[u8;8* 4]>){
	for row in 0..4 {
    		for col in 0..8{
        		for bit in 0..8 {
            			if (arrays[row * col + ] & (0b1 << bit)) > 0{
                			if (x * y + x)
                    				self.put(bit as i32, y as i32);
                    			}else {
                				self.put((x + bit) as i32, y as i32);
                    			}
            			}
        		}
    		}
	}
        self.present();
    }
}
