use std::io::Read;
use std::fs::File;

mod window;
mod cpu;
mod ram;
mod instruction;
//mod font;

use instruction::Instruction;


const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;

#[allow(dead_code)]
#[derive(PartialEq)]
enum State {
    Quit,
    Running,
    Paused
}


#[allow(dead_code)]
pub struct Machine {
    state: State,
    ram: [u8; 4096],
    window: window::Window,
    display: [bool; 2048],      // emulate original CHIP-8 resolution pixels
    stack: [u16; 12],           // subroutine stack
    stack_ptr: usize,           // subroutine stack pointer
    v: [u8; 16],                // data registers V0-VF
    i: u16,                     // index register
    pc: u16,                    // program counter
    delay_timer: u8,            // decrements at 60hz while > 0
    sound_timer: u8,            // decrements at 60hz while and play tone when > 0
    keypad: [bool; 16],         // hexadecimal keypad 0x0 - 0xF
    rom_name: String,           // currently running rom
    instruction: Instruction    // curerntly executing instruction
}


#[allow(dead_code)]
impl Machine {
    pub fn new() -> Self {
        Self {
            state: State::Paused,
            ram: [0; 0x1000],
            window: window::Window::new(String::from("CHIP-8 EMU"), 600, 300),
            display: [false; 2048],
            stack: [0; 12],
            stack_ptr: 0,
            v: [0; 16],
            i: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            rom_name: String::new(),
            instruction: Instruction::new()
        }
    }

    pub fn init(&mut self, rom_name: &String) -> Result<&mut Self, ()> {
        let entry_point: u16 = 0x200;       // rom will be loaded to 0x200 addr of ram
        let entry_point_usize: usize = 0x200;

        // TODO: load font
        self.ram[0..font::FONT_SET.len()].copy_from_slice(&font::FONT_SET);

        // TODO: load rom
        let mut rom_file: File = File::open(rom_name).expect("Failed to open rom file!");

        let rom_size: usize = rom_file
            .read(&mut self.ram[entry_point_usize..])
            .unwrap();

        let max_size: usize = self.ram.len() - entry_point_usize;

        if rom_size > max_size {
            panic!("What the fuck?! Why does this rom filesize is bigger than RAM!??\n
                    Are you trying to load GTA V on chip-8 emulator or what?")
        }

        self.state = State::Running;
        self.rom_name = rom_name.clone();
        self.pc = entry_point;              // start program counter at rom entry point

        Ok(self)
    }

    pub fn exec_instruction(&mut self) {
        self.instruction = Instruction::from(
            self.ram[self.pc as usize],
            self.ram[self.pc as usize + 1]
        );

        let category: u16 = (self.instruction.opcode >> 12) & 0x0F;

        print!("Address: {:#06x}, Opcode: {:#06x}, Desc: ", self.pc, self.instruction.opcode);

        self.pc += 2;

        match category {
            0x00 => {
                if self.instruction.nn == 0xE0 {
                    println!("clear screen");
                    self.display.fill_with(|| false);
                } else if self.instruction.nn == 0xEE {
                    println!("return from subroutine");
                    // 0x00EE return from subroutine
                    // set program counter to last address from subroutine stack ("pop" it from stack)
                    //  so that next opcode will be gotten from that address
                    self.stack_ptr -= 1;
                    self.pc = self.stack[self.stack_ptr];
                } else {
                    println!("unimplemented opcode 0x00");
                }
            }
            0x01 => {
                println!("jump to addres nnn ({:#06x})", self.instruction.nnn);
                self.pc = self.instruction.nnn;
            }
            0x02 => {
                println!("return to subroutine stack");
                // store current addres to return to subroutine stack
                //  and set program counter to subroutine address so that the next opcode
                //  is gotten from there
                self.stack[self.stack_ptr] = self.pc;
                self.stack_ptr += 1;
                self.pc = self.instruction.nnn;
            }
            0x06 => {
                println!("set v[{}] to nn ({:#04x})", self.instruction.x, self.instruction.nn);
                self.v[self.instruction.x as usize] = self.instruction.nn;
            }
            0x07 => {
                println!("set v[{}] += nn ({:#04x})", self.instruction.x, self.instruction.nn);
                self.v[self.instruction.x as usize] += self.instruction.nn;
            }
            0x0A => {
                println!("set i to nnn");
                self.i = self.instruction.nnn;
            }
            0x0D => {
                println!("draw a N {} height srite at coords v{} ({:#04x}), v{} ({:#04x}) from mem loc i {:#06x}",
                    self.instruction.n,
                    self.instruction.x,
                    self.v[self.instruction.x as usize],
                    self.instruction.y,
                    self.v[self.instruction.y as usize],
                    self.i);

                let mut x_coord: u8 = self.v[self.instruction.x as usize] % WIDTH;
                let mut y_coord: u8 = self.v[self.instruction.y as usize] % HEIGHT;
                let origin_x_coord: u8 = x_coord;

                self.v[0xF] = 0;    // initialize carry flag to 0

                for i in 0..self.instruction.n {
                    let sprite_data: u8 = self.ram[(self.i + i as u16) as usize];
                    x_coord = origin_x_coord;

                    for j in (0..8).rev() {
                        let pixel: &mut bool = &mut self.display[(y_coord as u16 * WIDTH as u16 + x_coord as u16) as usize];
                        let sprite_bit: bool = (sprite_data & (1 << j)) != 0;

                        if sprite_bit && *pixel {
                            self.v[0xF] = 1;
                        }

                        *pixel ^= sprite_bit;
                        x_coord += 1;

                        if x_coord >= WIDTH {
                            break
                        }
                    }

                    y_coord += 1;

                    if y_coord >= HEIGHT {
                        break
                    }
                }

                
            }
            _ => { println!("unimplemented opcode"); }
        }
    }

    pub fn tick(&mut self) {
        // TODO: handle input
        self.exec_instruction();
        self.window.update_screen(&self.display);

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    pub fn is_running(&self) -> bool {
        self.state == State::Running
    }
}