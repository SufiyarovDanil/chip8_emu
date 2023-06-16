use std::time;
use piston_window::{
    Button,
    Key
};

mod window;
mod cpu;
mod ram;
mod display;


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
    ram: ram::RAM,
    window: window::Window,
    display: display::Display,  // emulate original CHIP-8 resolution pixels
    cpu: cpu::CPU,
    keypad: [bool; 16],         // hexadecimal keypad 0x0 - 0xF
    rom_name: String,           // currently running rom
}


#[allow(dead_code)]
impl Machine {
    pub fn new() -> Self {
        Self {
            state: State::Paused,
            ram: ram::RAM::new(),
            window: window::Window::new(String::from("CHIP-8 EMU"), 1200, 600),
            display: display::Display::new(),
            cpu: cpu::CPU::new(),
            keypad: [false; 16],
            rom_name: String::new(),
        }
    }

    pub fn init(&mut self, rom_name: String) -> &mut Self {
        self.cpu.bind_ram(&mut self.ram);
        self.cpu.bind_display(&mut self.display);
        self.cpu.bind_keypad(&mut self.keypad);
        self.ram.load_rom(rom_name.clone());
        self.state = State::Running;

        self
    }

    pub fn run(&mut self) {
        while self.state != State::Quit {
            let begin_time: time::Instant = time::Instant::now();

            self.handle_input();
            self.cpu.make_cycle();
            self.window.update_screen(&mut self.display);

            let mut end_time: time::Instant = time::Instant::now();

            while end_time.duration_since(begin_time).as_micros() < 16670 {
                end_time =  time::Instant::now();
            }
        }
    }

    fn handle_input(&mut self) {
        match self.window.get_presssed_key() {
            Button::Keyboard(Key::Escape) => { self.state = State::Quit }

            Button::Keyboard(Key::D1) => { self.keypad[0x1] = true; }
            Button::Keyboard(Key::D2) => { self.keypad[0x2] = true; }
            Button::Keyboard(Key::D3) => { self.keypad[0x3] = true; }
            Button::Keyboard(Key::D4) => { self.keypad[0xC] = true; }

            Button::Keyboard(Key::Q) => { self.keypad[0x4] = true; }
            Button::Keyboard(Key::W) => { self.keypad[0x5] = true; }
            Button::Keyboard(Key::E) => { self.keypad[0x6] = true; }
            Button::Keyboard(Key::R) => { self.keypad[0xD] = true; }

            Button::Keyboard(Key::A) => { self.keypad[0x7] = true; }
            Button::Keyboard(Key::S) => { self.keypad[0x8] = true; }
            Button::Keyboard(Key::D) => { self.keypad[0x9] = true; }
            Button::Keyboard(Key::F) => { self.keypad[0xE] = true; }

            Button::Keyboard(Key::Z) => { self.keypad[0xA] = true; }
            Button::Keyboard(Key::X) => { self.keypad[0x0] = true; }
            Button::Keyboard(Key::C) => { self.keypad[0xB] = true; }
            Button::Keyboard(Key::V) => { self.keypad[0xF] = true; }
            _ => ()
        }

        match self.window.get_released_key() {
            Button::Keyboard(Key::D1) => { self.keypad[0x1] = false; }
            Button::Keyboard(Key::D2) => { self.keypad[0x2] = false; }
            Button::Keyboard(Key::D3) => { self.keypad[0x3] = false; }
            Button::Keyboard(Key::D4) => { self.keypad[0xC] = false; }

            Button::Keyboard(Key::Q) => { self.keypad[0x4] = false; }
            Button::Keyboard(Key::W) => { self.keypad[0x5] = false; }
            Button::Keyboard(Key::E) => { self.keypad[0x6] = false; }
            Button::Keyboard(Key::R) => { self.keypad[0xD] = false; }

            Button::Keyboard(Key::A) => { self.keypad[0x7] = false; }
            Button::Keyboard(Key::S) => { self.keypad[0x8] = false; }
            Button::Keyboard(Key::D) => { self.keypad[0x9] = false; }
            Button::Keyboard(Key::F) => { self.keypad[0xE] = false; }

            Button::Keyboard(Key::Z) => { self.keypad[0xA] = false; }
            Button::Keyboard(Key::X) => { self.keypad[0x0] = false; }
            Button::Keyboard(Key::C) => { self.keypad[0xB] = false; }
            Button::Keyboard(Key::V) => { self.keypad[0xF] = false; }
            _ => ()
        }
    }
}