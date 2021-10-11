use crate::instruction::Instruction;
use crate::rom::rom::load_rom;

use rand::Rng;

pub const START_ADDRESS: u16 = 0x200;
const FONT_START_ADDRESS: u16 = 0x50;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Cpu {
    // 0x000 to 0x1ff unused
    // programs usually start from 0x200 but sometimes 0x600
    mem: [u8; 4096],
    // general purpose registers
    v: [u8; 16],
    // used to store memory addresses
    i: u16,
    // used for delay and sound timers
    // when these are non zero they decrement at 60hz
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    pub pixels: [[bool; 64]; 32],
    pub should_draw: bool,
    pub keys: [bool; 16],
}

impl Cpu {
    pub fn init() -> Cpu {
        let mut cpu = Cpu {
            mem: [0; 4096],
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: START_ADDRESS,
            sp: 0,
            stack: [0; 16],
            pixels: [[false; WIDTH]; HEIGHT],
            should_draw: true,
            keys: [false; 16],
        };
        cpu.load_fonts();

        cpu
    }

    fn load_fonts(&mut self) {
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        let start = FONT_START_ADDRESS as usize;
        let end = start + fonts.len();
        self.mem[start..end].clone_from_slice(&fonts);
    }

    pub fn load_rom(&mut self, filename: &String) {
        load_rom(filename, &mut self.mem);
    }

    pub fn cycle(&mut self) {
        let pc = self.pc as usize;
        let bytes = (self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16;

        self.pc += 2;
        match Instruction::parse(bytes) {
            Ok(inst) => {
                println!("{:x?}: {:x?} {:?}", pc, bytes, inst);
                self.execute(inst);
            }
            Err(error) => {
                panic!(
                    "Failed to execute instruction at {:x?} due to {:?}",
                    pc, error
                );
            }
        }
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SYS(_) => (),
            Instruction::CLS => {
                self.pixels = [[false; WIDTH]; HEIGHT];
                self.should_draw = true;
            }
            Instruction::RET => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Instruction::JP_ADDR(addr) => {
                self.pc = addr;
            }
            Instruction::CALL_ADDR(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Instruction::SE_BYTE(addr, byte) => {
                if self.v[addr as usize] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SNE_BYTE(addr, byte) => {
                if self.v[addr as usize] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SE(addr_x, addr_y) => {
                let vx = self.v[addr_x as usize];
                let vy = self.v[addr_y as usize];

                if vx == vy {
                    self.pc += 2;
                }
            }
            Instruction::LD_BYTE(addr, byte) => {
                self.v[addr as usize] = byte;
            }
            Instruction::ADD_BYTE(addr, byte) => {
                let (result, _) = self.v[addr as usize].overflowing_add(byte);
                self.v[addr as usize] = result;
            }
            Instruction::LD(addr_x, addr_y) => {
                self.v[addr_x as usize] = self.v[addr_y as usize];
            }
            Instruction::OR(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                self.v[x] |= self.v[y];
            }
            Instruction::AND(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                self.v[x] &= self.v[y];
            }
            Instruction::XOR(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                self.v[x] ^= self.v[y];
            }
            Instruction::ADD(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                self.v[0x0F] = overflow as u8;
            }
            Instruction::SUB(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                self.v[0x0F] = match self.v[x] > self.v[y] {
                    true => 1,
                    false => 0,
                };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }
            Instruction::SHR(addr) => {
                let x = addr as usize;

                self.v[0xf] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            }
            Instruction::SUBN(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                self.v[0x0F] = match self.v[y] > self.v[x] {
                    true => 1,
                    false => 0,
                };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }
            Instruction::SHL(addr) => {
                let x = addr as usize;

                self.v[0xf] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            Instruction::SNE(addr_x, addr_y) => {
                let x = addr_x as usize;
                let y = addr_y as usize;

                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            Instruction::LD_I(addr) => {
                self.i = addr;
            }
            Instruction::JP_V0(addr) => {
                self.pc = addr + self.v[0] as u16;
            }
            Instruction::RND_BYTE(addr, byte) => {
                let x = addr as usize;

                let mut rng = rand::thread_rng();
                let rand: u8 = rng.gen();

                self.v[x] = rand & byte;
            }
            Instruction::DRW(addr_x, addr_y, n) => {
                let vx = self.v[addr_x as usize] as usize % WIDTH;
                let vy = self.v[addr_y as usize] as usize % HEIGHT;

                let start = self.i as usize;
                let end = start + n as usize;

                let sprite = &self.mem[start..end];

                self.v[0xf] = 0;
                for (row, sprite_byte) in sprite.iter().enumerate() {
                    for col in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                        let screen_pixel = self.pixels[vy + row][vx + col] as u8;
                        let old_pixel = screen_pixel;

                        if screen_pixel == 1 && sprite_pixel == 1 {
                            self.v[0xf] = 1;
                        }
                        self.pixels[vy + row][vx + col] = (screen_pixel ^ sprite_pixel) != 0;
                        if self.pixels[vy + row][vx + col] != (old_pixel == 1) {
                            self.should_draw = true;
                        }
                    }
                }
            }
            Instruction::SKP(addr) => {
                let key = self.v[addr as usize] as usize;

                if self.keys[key] == true {
                    self.pc += 2;
                }
            }
            Instruction::SKNP(addr) => {
                let key = self.v[addr as usize] as usize;

                if self.keys[key] == false {
                    self.pc += 2;
                }
            }
            Instruction::LD_DT(addr) => {
                self.v[addr as usize] = self.dt;
            }
            Instruction::LD_KEY(addr) => {
                let x = addr as usize;
                let mut wait = true;

                for (key, pressed) in self.keys.iter().enumerate() {
                    if *pressed == true {
                        self.v[x] = key as u8;
                        wait = false;
                        break;
                    }
                }

                if wait {
                    self.pc -= 2;
                }
            }
            Instruction::LD_DT_SET(addr) => {
                self.dt = self.v[addr as usize];
            }
            Instruction::LD_ST_SET(addr) => {
                self.st = self.v[addr as usize];
            }
            Instruction::ADD_I(addr) => {
                let vx = self.v[addr as usize] as u16;
                self.i = self.i.wrapping_add(vx);
            }
            Instruction::LD_F(addr) => {
                let vx = self.v[addr as usize] as usize;
                self.i = FONT_START_ADDRESS + (vx * 5) as u16;
            }
            Instruction::LD_B(addr) => {
                let mut vx = self.v[addr as usize] as usize;

                self.mem[(self.i + 2) as usize] = (vx % 10) as u8;
                vx /= 10;

                self.mem[(self.i + 1) as usize] = (vx % 10) as u8;
                vx /= 10;

                self.mem[(self.i) as usize] = (vx % 10) as u8;
            }
            Instruction::LD_STORE_I(addr) => {
                let vx = addr as usize + 1;
                let start = self.i as usize;
                let end = start + vx;

                self.mem[start..end].clone_from_slice(&self.v[0..vx]);
            }
            Instruction::LD_READ_I(addr) => {
                let vx = addr as usize + 1;
                let start = self.i as usize;
                let end = start + vx;

                self.v[0..vx].clone_from_slice(&self.mem[start..end]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cls() {
        let mut cpu = Cpu::init();
        cpu.pixels = [[true; WIDTH]; HEIGHT];
        cpu.execute(Instruction::CLS);

        assert_eq!(cpu.pixels, [[false; WIDTH]; HEIGHT]);
    }

    #[test]
    fn test_ret() {
        let mut cpu = Cpu::init();
        cpu.stack[0] = 0x123;
        cpu.pc = 0xFFF;
        cpu.sp = 1;
        cpu.execute(Instruction::RET);

        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 0x123);
    }

    #[test]
    fn test_jp_addr() {
        let mut cpu = Cpu::init();
        cpu.execute(Instruction::JP_ADDR(0x0123));

        assert_eq!(cpu.pc, 0x0123);
    }

    #[test]
    fn test_call_addr() {
        let mut cpu = Cpu::init();
        cpu.sp = 0;
        cpu.pc = 0x0ABC;
        cpu.execute(Instruction::CALL_ADDR(0x0123));

        assert_eq!(cpu.stack[0], 0xABC);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0x0123);
    }

    #[test]
    fn test_se_byte_equal() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 0x12;
        cpu.execute(Instruction::SE_BYTE(1, 0x012));

        assert_eq!(cpu.pc, 2 + START_ADDRESS);
    }

    #[test]
    fn test_se_byte_not_equal() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 0x12;
        cpu.execute(Instruction::SE_BYTE(1, 0));

        assert_eq!(cpu.pc, 0 + START_ADDRESS);
    }

    #[test]
    fn test_sne_byte_equal() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 0x12;
        cpu.execute(Instruction::SNE_BYTE(1, 0x12));

        assert_eq!(cpu.pc, 0 + START_ADDRESS);
    }

    #[test]
    fn test_sne_byte_not_equal() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 0x12;
        cpu.execute(Instruction::SNE_BYTE(0x0001, 0x00));

        assert_eq!(cpu.pc, 2 + START_ADDRESS);
    }

    #[test]
    fn test_se_equal() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x12;
        cpu.v[1] = 0x12;
        cpu.execute(Instruction::SE(0, 1));

        assert_eq!(cpu.pc, 2 + START_ADDRESS);
    }

    #[test]
    fn test_se_not_equal() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x12;
        cpu.v[1] = 0x34;
        cpu.execute(Instruction::SE(0, 1));

        assert_eq!(cpu.pc, 0 + START_ADDRESS);
    }

    #[test]
    fn test_ld_byte() {
        let mut cpu = Cpu::init();
        cpu.execute(Instruction::LD_BYTE(0, 0x12));

        assert_eq!(cpu.v[0], 0x12);
    }

    #[test]
    fn test_add_byte() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x01;
        cpu.execute(Instruction::ADD_BYTE(0, 0x01));

        assert_eq!(cpu.v[0], 0x02);
    }

    #[test]
    fn test_ld() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x02;
        cpu.execute(Instruction::LD(0, 1));

        assert_eq!(cpu.v[0], 0x02);
        assert_eq!(cpu.v[1], 0x02);
    }

    #[test]
    fn test_or() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x10;
        cpu.execute(Instruction::OR(0, 1));

        assert_eq!(cpu.v[0], 0x11);
    }

    #[test]
    fn test_and() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x01;
        cpu.v[1] = 0x11;
        cpu.execute(Instruction::AND(0, 1));

        assert_eq!(cpu.v[0], 0x01);
    }

    #[test]
    fn test_xor() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x11;
        cpu.v[1] = 0x10;
        cpu.execute(Instruction::XOR(0, 1));

        assert_eq!(cpu.v[0], 0x01);
    }

    #[test]
    fn test_add_no_carry() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0xFE;
        cpu.v[1] = 0x01;
        cpu.execute(Instruction::ADD(0, 1));

        assert_eq!(cpu.v[0], 0xFF);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0xFF;
        cpu.v[1] = 0x01;
        cpu.execute(Instruction::ADD(0, 1));

        assert_eq!(cpu.v[0], 0x00);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_sub_vx_eq_vy() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 1;
        cpu.execute(Instruction::SUB(0, 1));

        assert_eq!(cpu.v[0], 0);
        assert_eq!(cpu.v[1], 1);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_sub_vx_lt_vy() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 2;
        cpu.execute(Instruction::SUB(0, 1));

        assert_eq!(cpu.v[0], 255);
        assert_eq!(cpu.v[1], 2);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_sub_vx_gt_vy() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 2;
        cpu.v[1] = 1;
        cpu.execute(Instruction::SUB(0, 1));

        assert_eq!(cpu.v[0], 1);
        assert_eq!(cpu.v[1], 1);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_shr_set_0() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0x02;
        cpu.execute(Instruction::SHR(0));

        assert_eq!(cpu.v[0], 0x01);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_shr_set_1() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.execute(Instruction::SHR(0));

        assert_eq!(cpu.v[0], 0x00);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_subn_vy_eq_vx() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 1;
        cpu.execute(Instruction::SUBN(0, 1));

        assert_eq!(cpu.v[0], 0);
        assert_eq!(cpu.v[1], 1);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_subn_vy_lt_vx() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 2;
        cpu.v[1] = 1;
        cpu.execute(Instruction::SUBN(0, 1));

        assert_eq!(cpu.v[0], 255);
        assert_eq!(cpu.v[1], 1);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_subn_vy_gt_vx() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 2;
        cpu.execute(Instruction::SUBN(0, 1));

        assert_eq!(cpu.v[0], 1);
        assert_eq!(cpu.v[1], 2);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_shl_set_0() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.execute(Instruction::SHL(0));

        assert_eq!(cpu.v[0], 2);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_shl_set_1() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 255;
        cpu.execute(Instruction::SHL(0));

        assert_eq!(cpu.v[0], 254);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_sne_eq() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 1;
        cpu.execute(Instruction::SNE(0, 1));

        assert_eq!(cpu.pc, START_ADDRESS);
    }

    #[test]
    fn test_sne_neq() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.v[1] = 2;
        cpu.execute(Instruction::SNE(0, 1));

        assert_eq!(cpu.pc, START_ADDRESS + 2);
    }

    #[test]
    fn test_ld_i() {
        let mut cpu = Cpu::init();
        cpu.execute(Instruction::LD_I(0x0FFF));

        assert_eq!(cpu.i, 0x0FFF);
    }

    #[test]
    fn test_jp_v0() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.execute(Instruction::JP_V0(1));

        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_rnd() {
        let mut cpu = Cpu::init();
        cpu.execute(Instruction::RND_BYTE(0, 0xFF));

        assert_ne!(cpu.v[0], 0);
    }

    #[test]
    fn test_drw_no_collision() {
        let mut cpu = Cpu::init();
        cpu.i = FONT_START_ADDRESS;
        cpu.v[0] = 0;
        cpu.v[1] = 0;
        cpu.execute(Instruction::DRW(0, 1, 1));

        assert_eq!(cpu.pixels[0][0], true);
        assert_eq!(cpu.v[0xf], 0);
    }

    #[test]
    fn test_drw_collision() {
        let mut cpu = Cpu::init();
        cpu.i = FONT_START_ADDRESS;
        cpu.v[0] = 0;
        cpu.v[1] = 0;
        cpu.execute(Instruction::DRW(0, 1, 1));
        cpu.execute(Instruction::DRW(0, 1, 1));

        assert_eq!(cpu.pixels[0][0], false);
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_skp_pressed() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.keys[1] = true;

        cpu.execute(Instruction::SKP(0));

        assert_eq!(cpu.pc, START_ADDRESS + 2);
    }

    #[test]
    fn test_skp_not_pressed() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;

        cpu.execute(Instruction::SKP(0));

        assert_eq!(cpu.pc, START_ADDRESS);
    }

    #[test]
    fn test_sknp_pressed() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;
        cpu.keys[1] = true;

        cpu.execute(Instruction::SKNP(0));

        assert_eq!(cpu.pc, START_ADDRESS);
    }

    #[test]
    fn test_sknp_not_pressed() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 1;

        cpu.execute(Instruction::SKNP(0));

        assert_eq!(cpu.pc, START_ADDRESS + 2);
    }

    #[test]
    fn test_ld_dt() {
        let mut cpu = Cpu::init();
        cpu.dt = 123;
        cpu.execute(Instruction::LD_DT(1));

        assert_eq!(cpu.v[1], 123);
    }

    #[test]
    fn test_ld_key_pressed() {
        let mut cpu = Cpu::init();
        cpu.keys[1] = true;
        cpu.v[0] = 1;
        cpu.execute(Instruction::LD_KEY(0));

        assert_eq!(cpu.v[0], 1);
        assert_eq!(cpu.pc, START_ADDRESS);
    }

    #[test]
    fn test_ld_key_not_pressed() {
        let mut cpu = Cpu::init();
        cpu.execute(Instruction::LD_KEY(0));

        assert_eq!(cpu.v[0], 0);
        assert_eq!(cpu.pc, START_ADDRESS - 2);
    }

    #[test]
    fn test_ld_dt_set() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 123;
        cpu.execute(Instruction::LD_DT_SET(1));

        assert_eq!(cpu.dt, 123);
    }

    #[test]
    fn test_ld_st_set() {
        let mut cpu = Cpu::init();
        cpu.v[1] = 123;
        cpu.execute(Instruction::LD_ST_SET(1));

        assert_eq!(cpu.st, 123);
    }

    #[test]
    fn test_add_i() {
        let mut cpu = Cpu::init();
        cpu.i = 1;
        cpu.v[1] = 1;
        cpu.execute(Instruction::ADD_I(1));

        assert_eq!(cpu.i, 2);
    }

    #[test]
    fn test_ld_f() {
        let mut cpu = Cpu::init();

        cpu.execute(Instruction::LD_F(0));
        assert_eq!(cpu.i, FONT_START_ADDRESS);

        cpu.v[0] = 1;
        cpu.execute(Instruction::LD_F(0));
        assert_eq!(cpu.i, FONT_START_ADDRESS + 5);
    }

    #[test]
    fn test_ld_b() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 123;

        cpu.execute(Instruction::LD_B(0));

        assert_eq!(cpu.mem[cpu.i as usize], 1);
        assert_eq!(cpu.mem[(cpu.i + 1) as usize], 2);
        assert_eq!(cpu.mem[(cpu.i + 2) as usize], 3);
    }

    #[test]
    fn test_ld_store_i() {
        let mut cpu = Cpu::init();
        cpu.v[0] = 0;
        cpu.v[1] = 1;
        cpu.v[2] = 2;

        cpu.execute(Instruction::LD_STORE_I(2));

        assert_eq!(cpu.mem[cpu.i as usize], 0);
        assert_eq!(cpu.mem[(cpu.i + 1) as usize], 1);
        assert_eq!(cpu.mem[(cpu.i + 2) as usize], 2);
    }

    #[test]
    fn test_ld_read_i() {
        let mut cpu = Cpu::init();

        cpu.mem[(cpu.i) as usize] = 0;
        cpu.mem[(cpu.i + 1) as usize] = 1;
        cpu.mem[(cpu.i + 2) as usize] = 2;

        cpu.execute(Instruction::LD_READ_I(2));

        assert_eq!(cpu.v[0], 0);
        assert_eq!(cpu.v[1], 1);
        assert_eq!(cpu.v[2], 2);
    }
}
