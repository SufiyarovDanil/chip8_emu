pub struct Instruction {
    pub opcode: u16,
    pub nnn: u16,       // 12 bit addr constant
    pub nn: u8,         // 8 bit constant
    pub n: u8,          // 4 bit constant
    pub x: usize,       // 4 bit register identifier
    pub y: usize        // 4 bit register identifier
}


impl Instruction {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            nnn: 0,
            nn: 0,
            n: 0,
            x: 0,
            y: 0
        }
    }

    pub fn from(opcode1: u8, opcode2: u8) -> Self {
        let left: u16 = (opcode1 as u16) << 8;
        let right: u16 = opcode2 as u16;
        let opcode = left | right;

        let nnn: u16 = opcode & 0x0FFF;
        let nn: u8 = opcode as u8 & 0x0FF;
        let n: u8 = (opcode & 0x0F) as u8;
        let x: usize = ((opcode >> 8)  & 0x0F) as usize;
        let y: usize = ((opcode >> 4) & 0x0F) as usize;

        Self {
            opcode,
            nnn,
            nn,
            n,
            x,
            y
        }
    }
}
