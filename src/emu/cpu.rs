pub const ENTRY_POINT: u16 = 0x200;

pub struct CPU {
    v: [u8; 16],                // data registers V0-VF
    i: u16,                     // index register
    pc: u16,                    // program counter
    stack: [u16; 12],           // subroutine stack
    stack_ptr: usize,           // subroutine stack pointer
}


impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: ENTRY_POINT,
            stack: [0; 12],
            stack_ptr: 0
        }
    }

    pub fn exec_instruction(opcode: u16,) {

    }

    // fn init_instruction() -> std::collections::HashMap<u16, u8> {

    // }

    fn x00(&mut self) {

    }
}
