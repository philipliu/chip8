use crate::cpu::Cpu;
use crate::display::Display;
use crate::input::Input;

use sdl2::Sdl;

pub struct Chip8 {
    cpu: Cpu,
    display: Display,
    input: Input,
    sdl_context: Sdl,
}

impl Chip8 {
    pub fn init() -> Chip8 {
        let sdl_context = sdl2::init().unwrap();

        Chip8 {
            cpu: Cpu::init(),
            display: Display::init(&sdl_context),
            input: Input::init(),
            sdl_context,
        }
    }

    pub fn load(&mut self, filename: &String) {
        self.cpu.load_rom(filename);
    }

    pub fn start(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        loop {
            for event in event_pump.poll_iter() {
                self.input.process(&mut self.cpu.keys, event);
            }
            self.cpu.cycle();
            if self.cpu.should_draw {
                self.display.render(&self.cpu.pixels);
                self.cpu.should_draw = false;
            }
            ::std::thread::sleep(std::time::Duration::new(0, 2000000));
        }
    }
}
