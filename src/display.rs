use std::convert::TryInto;

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::WIDTH;
use crate::cpu::HEIGHT;

const SCALE: usize = 10;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn init(sdl_context: &sdl2::Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = ((WIDTH * SCALE) as u32).try_into().unwrap();
        let window_height = ((HEIGHT * SCALE) as u32).try_into().unwrap();

        let window = video_subsystem
            .window("chip8", window_width, window_height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, pixel: &bool) {
        let color = match pixel {
            true => Color::RGB(255, 255, 255),
            false => Color::RGB(0, 0, 0),
        };

        self.canvas.set_draw_color(color);
        let _ = self
            .canvas
            .fill_rect(Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE as u32, SCALE as u32));
    }

    pub fn render(&mut self, pixels: &[[bool; WIDTH]; HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                self.draw_pixel(x, y, pixel);
            }
        }
        self.canvas.present();
    }
}
