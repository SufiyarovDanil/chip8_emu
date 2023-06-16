mod instruction;

use std::collections::HashMap;
use instruction::Instruction;
use super::display;
use super::ram;

pub const ENTRY_POINT: u16 = 0x200;
const INSTRUCTIONS_PER_SECOND: u32 = 500;


pub struct CPU {
    v: [u8; 16],                // data registers V0-VF
    i: u16,                     // index register
    pc: u16,                    // program counter
    stack: [u16; 12],           // subroutine stack
    stack_ptr: usize,           // subroutine stack pointer
    delay_timer: u8,            // decrements at 60hz while > 0
    sound_timer: u8,            // decrements at 60hz while and play tone when > 0
    instruction_map: HashMap<u16, fn(&mut Self)>,
    current_inst: Instruction,
    ram_handler: *mut ram::RAM,
    display_handler: *mut display::Display,
    keypad_handler: *mut [bool; 16]
}


impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: ENTRY_POINT,
            stack: [0; 12],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,
            instruction_map: Self::create_instruction_map(),
            current_inst: Instruction::new(),
            ram_handler: std::ptr::null_mut(),
            display_handler: std::ptr::null_mut(),
            keypad_handler: std::ptr::null_mut()
        }
    }

    pub fn bind_ram(&mut self, ram: *mut ram::RAM) {
        self.ram_handler = ram;
    }

    pub fn bind_display(&mut self, display: *mut display::Display) {
        self.display_handler = display;
    }

    pub fn bind_keypad(&mut self, keypad: *mut [bool; 16]) {
        self.keypad_handler = keypad;
    }

    pub fn make_cycle(&mut self) {
        for _ in 0..INSTRUCTIONS_PER_SECOND / 60 {
            self.exec_instruction();
        }

        self.update_timers();
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            // TODO: play sound
        }
        else {
            // TODO: stop playing sound
        }
    }

    fn exec_instruction(&mut self) {
        if self.ram_handler.is_null() || self.display_handler.is_null() {
            panic!("nullpointer exception");
        }

        unsafe {
            self.current_inst = Instruction::from(
                (*self.ram_handler).read(self.pc as usize),
                (*self.ram_handler).read(self.pc as usize + 1)
            );

            //println!("{:#06x}", self.pc);

            self.pc += 2;
    
            let category: u16 = (self.current_inst.opcode >> 12) & 0x0F;
            
            let exec: Option<&fn(&mut CPU)> = self
                .instruction_map
                .get(&category);

            if let Some(e) = exec {
                e(self);
            } else {
                println!("----------unimplemented instruction----------");
            }
        }
    }

    fn create_instruction_map() -> HashMap<u16, fn(&mut Self)> {
        let arr: [(u16, fn(&mut Self)); 16] = [
            (0x00, Self::x00),
            (0x01, Self::x01),
            (0x02, Self::x02),
            (0x03, Self::x03),
            (0x04, Self::x04),
            (0x05, Self::x05),
            (0x06, Self::x06),
            (0x07, Self::x07),
            (0x08, Self::x08),
            (0x09, Self::x09),
            (0x0A, Self::x0a),
            (0x0B, Self::x0b),
            (0x0C, Self::x0c),
            (0x0D, Self::x0d),
            (0x0E, Self::x0e),
            (0x0F, Self::x0f)
        ];

        HashMap::from(arr)
    }

    fn x00(&mut self) {
        //println!("x00");

        match self.current_inst.nn {
            0xE0 => unsafe {
                //println!("clear screen");

                (*self.display_handler).clear();
            }
            0xEE => {
                //println!("return from subroutine");

                self.stack_ptr -= 1;
                self.pc = self.stack[self.stack_ptr];
            }
            _ => { println!("unimplemented code"); }
        }
    }

    fn x01(&mut self) {
        //println!("jump to addres nnn ({:#06x})", self.current_inst.nnn);

        self.pc = self.current_inst.nnn;
    }

    fn x02(&mut self) {
        //println!("return to subroutine stack");
        // store current addres to return to subroutine stack
        //  and set program counter to subroutine address so that the next opcode
        //  is gotten from there
        self.stack[self.stack_ptr] = self.pc;
        self.stack_ptr += 1;
        self.pc = self.current_inst.nnn;
    }

    fn x03(&mut self) {
        //println!("x03");

        if self.v[self.current_inst.x] == self.current_inst.nn {
            self.pc += 2;
        }
    }

    fn x04(&mut self) {
        //println!("x04");

        if self.v[self.current_inst.x] != self.current_inst.nn {
            self.pc += 2;
        }
    }

    fn x05(&mut self) {
        //println!("x05");

        if self.current_inst.n != 0 {
            println!("wrong opcode");
            return;
        }

        if self.v[self.current_inst.x] == self.v[self.current_inst.y] {
            self.pc += 2;
        }
    }

    fn x06(&mut self) {
        //println!("set v[{}] to nn ({:#04x})", self.current_inst.x, self.current_inst.nn);

        self.v[self.current_inst.x] = self.current_inst.nn;
    }

    fn x07(&mut self) {
        //println!("set v[{}] += nn ({:#04x})", self.current_inst.x, self.current_inst.nn);

        self.v[self.current_inst.x] = self.v[self.current_inst.x]
            .wrapping_add(self.current_inst.nn); //+= self.current_inst.nn;
    }

    fn x08(&mut self) {
        //println!("x08");

        match self.current_inst.n {
            0x0 => {
                self.v[self.current_inst.x] = self.v[self.current_inst.y];
            }
            0x1 => {
                self.v[self.current_inst.x] |= self.v[self.current_inst.y];
                self.v[0xF] = 0; // IDK maybe i should remove it
            }
            0x2 => {
                self.v[self.current_inst.x] &= self.v[self.current_inst.y];
                self.v[0xF] = 0; // IDK maybe i should remove it
            }
            0x3 => {
                self.v[self.current_inst.x] ^= self.v[self.current_inst.y];
                self.v[0xF] = 0; // IDK maybe i should remove it
            }
            0x4 => {
                self.v[0xF] = (self.v[self.current_inst.x] as u16 + self.v[self.current_inst.y] as u16 > 255) as u8;

                self.v[self.current_inst.x] = self.v[self.current_inst.x]
                    .wrapping_add(self.v[self.current_inst.y]);
            }
            0x5 => {
                self.v[0xF] = (self.v[self.current_inst.y] <= self.v[self.current_inst.x]) as u8;

                self.v[self.current_inst.x] = self.v[self.current_inst.x]
                    .wrapping_sub(self.v[self.current_inst.y]);
            }
            0x6 => {
                self.v[0xF] = self.v[self.current_inst.x] & 0x1;
                self.v[self.current_inst.x] >>= 1;
            }
            0x7 => {
                self.v[0xF] = (self.v[self.current_inst.x] <= self.v[self.current_inst.y]) as u8;

                self.v[self.current_inst.x] = self.v[self.current_inst.y]
                    .wrapping_sub(self.v[self.current_inst.x]);
            }
            0xE => {
                self.v[0xF] = (self.v[self.current_inst.x] & 0x80) >> 7;
                self.v[self.current_inst.x] <<= 1;
            }

            _ => {
                println!("wrong opcode");
            }
        }

        
    }

    fn x09 (&mut self) {
        //println!("x09");

        if self.v[self.current_inst.x] != self.v[self.current_inst.y] {
            self.pc += 2;
        }
    }

    fn x0a(&mut self) {
        //println!("set i to nnn ({:#06x})", self.current_inst.nnn);

        self.i = self.current_inst.nnn;
    }

    fn x0b(&mut self) {
        //println!("x0b");

        self.pc = self.v[0] as u16 + self.current_inst.nnn;
    }

    fn x0c(&mut self) {
        //println!("x0c");

        self.v[self.current_inst.x] = rand::random::<u8>() & self.current_inst.nn;
    }

    fn x0d(&mut self) {
        //println!("draw a N {} height srite at coords v{} ({:#04x}), v{} ({:#04x}) from mem loc i {:#06x}",
        //    self.current_inst.n,
        //    self.current_inst.x,
        //    self.v[self.current_inst.x as usize],
        //    self.current_inst.y,
        //    self.v[self.current_inst.y as usize],
        //    self.i);

        let mut x_coord: u8 = self.v[self.current_inst.x as usize] % display::WIDTH;
        let mut y_coord: u8 = self.v[self.current_inst.y as usize] % display::HEIGHT;
        let origin_x_coord: u8 = x_coord;

        self.v[0xF] = 0;    // initialize carry flag to 0

        for i in 0..self.current_inst.n {
            let sprite_data: u8 = unsafe{ (*self.ram_handler).read((self.i + i as u16) as usize) };
            x_coord = origin_x_coord;

            for j in (0..8).rev() {
                let mut pixel: bool = unsafe {
                    (*self.display_handler)
                        .read_pixel((y_coord as u16 * display::WIDTH as u16 + x_coord as u16) as usize)
                };

                let sprite_bit: bool = (sprite_data & (1 << j)) != 0;

                if sprite_bit && pixel {
                    self.v[0xF] = 1;
                }

                pixel ^= sprite_bit;

                unsafe {
                    (*self.display_handler)
                        .write_pixel((y_coord as u16 * display::WIDTH as u16 + x_coord as u16) as usize, pixel);
                }

                x_coord += 1;

                if x_coord >= display::WIDTH {
                    break
                }
            }

            y_coord += 1;

            if y_coord >= display::HEIGHT {
                break
            }
        }
    }

    fn x0e(&mut self) {
        //println!("x0e");
        if self.keypad_handler.is_null() {
            return
        }

        match self.current_inst.nn {
            0x9E => unsafe {
                if (*self.keypad_handler)[self.v[self.current_inst.x] as usize] {
                    self.pc += 2;
                }
            }
            0xA1 => unsafe {
                if !(*self.keypad_handler)[self.v[self.current_inst.x] as usize] {
                    self.pc += 2;
                }
            }
            _ => ()
        }
    }

    fn x0f(&mut self) {
        //println!("x0f");

        match self.current_inst.nn {
            0x0A => unsafe {
                let mut any_key_pressed: bool = false;

                for i in 0..(*self.keypad_handler).len() {
                    if (*self.keypad_handler)[i] {
                        self.v[self.current_inst.x] = i as u8;
                        any_key_pressed = true;

                        break;
                    }
                }

                if !any_key_pressed {
                    self.pc -= 2;
                }
            }
            0x1E => {
                self.i += self.v[self.current_inst.x] as u16;
            }
            0x07 => {
                self.v[self.current_inst.x] = self.delay_timer;
            }
            0x15 => {
                self.delay_timer = self.v[self.current_inst.x];
            }
            0x18 => {
                self.sound_timer = self.v[self.current_inst.x];
            }
            0x29 => {
                self.i = self.v[self.current_inst.x].wrapping_mul(5) as u16;
            }
            0x33 => unsafe {
                let mut bcd: u8 = self.v[self.current_inst.x];

                (*self.ram_handler).write(self.i as usize + 2, bcd %10);
                bcd /= 10;
                (*self.ram_handler).write(self.i as usize + 1, bcd %10);
                bcd /= 10;
                (*self.ram_handler).write(self.i as usize, bcd);
            }
            0x55 => unsafe {
                for i in 0..self.current_inst.x + 1 {
                    (*self.ram_handler).write(self.i as usize + i, self.v[i]);
                }
            }
            0x65 => unsafe {
                for i in 0..self.current_inst.x + 1 {
                    self.v[i] = (*self.ram_handler).read(self.i as usize + i);
                }
            }
            _ => ()
        }
    }
}
