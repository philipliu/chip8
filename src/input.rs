use sdl2::event::Event::{self, KeyDown, KeyUp};
use sdl2::keyboard::Keycode;

pub struct Input {
}

impl Input {
    pub fn init() -> Input {
        Input {}
    }

    pub fn process(&self, keys: &mut [bool; 16], event: Event) {
        match event {
            KeyDown { keycode: Some(Keycode::X), .. } => {
                keys[0] = true;
            }
            KeyDown { keycode: Some(Keycode::Num1), .. } => {
                keys[1] = true;
            }
            KeyDown { keycode: Some(Keycode::Num2), .. } => {
                keys[2] = true;
            }
            KeyDown { keycode: Some(Keycode::Num3), .. } => {
                keys[3] = true;
            }
            KeyDown { keycode: Some(Keycode::Q), .. } => {
                keys[4] = true;
            }
            KeyDown { keycode: Some(Keycode::W), .. } => {
                keys[5] = true;
            }
            KeyDown { keycode: Some(Keycode::E), .. } => {
                keys[6] = true;
            }
            KeyDown { keycode: Some(Keycode::A), .. } => {
                keys[7] = true;
            }
            KeyDown { keycode: Some(Keycode::S), .. } => {
                keys[8] = true;
            }
            KeyDown { keycode: Some(Keycode::D), .. } => {
                keys[9] = true;
            }
            KeyDown { keycode: Some(Keycode::Z), .. } => {
                keys[0xA] = true;
            }
            KeyDown { keycode: Some(Keycode::C), .. } => {
                keys[0xB] = true;
            }
            KeyDown { keycode: Some(Keycode::Num4), .. } => {
                keys[0xC] = true;
            }
            KeyDown { keycode: Some(Keycode::R), .. } => {
                keys[0xD] = true;
            }
            KeyDown { keycode: Some(Keycode::F), .. } => {
                keys[0xE] = true;
            }
            KeyDown { keycode: Some(Keycode::V), .. } => {
                keys[0xF] = true;
            }
            KeyUp { keycode: Some(Keycode::X), .. } => {
                keys[0] = false;
            }
            KeyUp { keycode: Some(Keycode::Num1), .. } => {
                keys[1] = false;
            }
            KeyUp { keycode: Some(Keycode::Num2), .. } => {
                keys[2] = false;
            }
            KeyUp { keycode: Some(Keycode::Num3), .. } => {
                keys[3] = false;
            }
            KeyUp { keycode: Some(Keycode::Q), .. } => {
                keys[4] = false;
            }
            KeyUp { keycode: Some(Keycode::W), .. } => {
                keys[5] = false;
            }
            KeyUp { keycode: Some(Keycode::E), .. } => {
                keys[6] = false;
            }
            KeyUp { keycode: Some(Keycode::A), .. } => {
                keys[7] = false;
            }
            KeyUp { keycode: Some(Keycode::S), .. } => {
                keys[8] = false;
            }
            KeyUp { keycode: Some(Keycode::D), .. } => {
                keys[9] = false;
            }
            KeyUp { keycode: Some(Keycode::Z), .. } => {
                keys[0xA] = false;
            }
            KeyUp { keycode: Some(Keycode::C), .. } => {
                keys[0xB] = false;
            }
            KeyUp { keycode: Some(Keycode::Num4), .. } => {
                keys[0xC] = false;
            }
            KeyUp { keycode: Some(Keycode::R), .. } => {
                keys[0xD] = false;
            }
            KeyUp { keycode: Some(Keycode::F), .. } => {
                keys[0xE] = false;
            }
            KeyUp { keycode: Some(Keycode::V), .. } => {
                keys[0xF] = false;
            }
            _ => {}
        }
    }
}