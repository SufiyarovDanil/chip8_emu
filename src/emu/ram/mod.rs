use std::{
    io::Read,
    fs::File
};
use super::cpu;

mod font;


pub struct RAM {
    space: [u8; 0x1000]
}


impl RAM {
    pub fn new() -> Self {
        Self {
            space: [0u8; 0x1000]
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.space[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        self.space[addr] = val;
    }

    fn load_font(&mut self) {
        self.space[0..font::FONT_SET.len()].copy_from_slice(&font::FONT_SET);
    }

    pub fn load_rom(&mut self, rom_path: String) {
        self.load_font();

        let mut rom_file: File = File::open(rom_path).unwrap();

        let rom_size: usize = rom_file
            .read(&mut self.space[cpu::ENTRY_POINT as usize..])
            .unwrap();

        let max_size: usize = self.space.len() - cpu::ENTRY_POINT as usize;

        if rom_size > max_size {
            panic!("What the fuck?! Why does this rom filesize is bigger than RAM!??\n
                    Are you trying to load GTA V on chip-8 emulator or what?")
        }
    }
}
