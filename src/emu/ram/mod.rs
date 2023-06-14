
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

    pub fn load_rom(&mut self, path: String) {
        //self.ram[0..font::FONT_SET.len()].copy_from_slice(&font::FONT_SET);
        self.space[0..]
    }
}
