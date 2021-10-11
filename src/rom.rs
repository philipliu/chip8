pub mod rom {
    use crate::cpu::START_ADDRESS;
    use std::ptr;

    pub fn load_rom(filename: &String, mem: &mut [u8; 4096]) {
        println!("Loading rom: {}", filename);
        match std::fs::read(filename) {
            Ok(bytes) => unsafe {
                for (idx, element) in bytes.into_iter().enumerate() {
                    ptr::write(&mut mem[idx + (START_ADDRESS as usize)], element);
                }
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
        println!("Loaded rom: {}", filename);
    }
}
