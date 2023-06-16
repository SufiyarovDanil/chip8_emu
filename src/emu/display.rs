pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

pub struct Display {
    display: [bool; 2048]
}


impl Display {
    pub fn new() -> Self {
        Self {
            display: [false; 2048]
        }
    }

    pub fn clear(&mut self) {
        self.display.fill_with(|| false);
    }

    pub fn read_pixel(&mut self, addr: usize) -> bool {
        self.display[addr]
    }

    pub fn write_pixel(&mut self, addr: usize, pixel: bool) {
        self.display[addr] = pixel;
    }

    pub fn get_size(&self) -> usize {
        self.display.len()
    }
}