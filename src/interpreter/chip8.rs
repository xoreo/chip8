use super::instruction::Instruction as Instr;

const MEM_SIZE: usize = 0x1000;
const N_REGISTERS: usize = 16;
const STACK_DEPTH: usize = 12;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8 {
    memory: [u8; MEM_SIZE],   // The memory
    V: [u8; N_REGISTERS],     // The general purpose registers
    I: u16,                   // The I register
    stack: [u8; STACK_DEPTH], // The stack

    pc: u16, // The program counter
    sp: u8,  // The stack pointer

    delay_timer: u16, // The delay timer
    sound_timer: u16, // The sound timer
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            V: [0; N_REGISTERS],
            I: 0,
            stack: [0; STACK_DEPTH],
            pc: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn execute_instruction(&mut self, i: Instr) {
        match i {
            Instr::I0NNN(a) => {}
            Instr::I00E0 => {}
            Instr::I00EE => {}
            Instr::I1NNN(a) => {}
            Instr::I2NNN(a) => {}
            Instr::I3XNN(x, b) => {}
            Instr::I4XNN(x, b) => {}
            Instr::I5XY0(x, y) => {}
            Instr::I6XNN(x, b) => {}
            Instr::I7XNN(x, b) => {}
            Instr::I8XY0(x, y) => {}
            Instr::I8XY1(x, y) => {}
            Instr::I8XY2(x, y) => {}
            Instr::I8XY3(x, y) => {}
            Instr::I8XY4(x, y) => {}
            Instr::I8XY5(x, y) => {}
            Instr::I8XY6(x, y) => {}
            Instr::I8XY7(x, y) => {}
            Instr::I8XYE(x, y) => {}
            Instr::I9XY0(x, y) => {}
            Instr::IANNN(a) => {}
            Instr::IBNNN(a) => {}
            Instr::ICNNN(x, b) => {}
            Instr::IDXYN(x, y, n) => {}
            Instr::IEX9E(x) => {}
            Instr::IEXA1(x) => {}
            Instr::IFX07(x) => {}
            Instr::IFX0A(x) => {}
            Instr::IFX15(x) => {}
            Instr::IFX18(x) => {}
            Instr::IFX1E(x) => {}
            Instr::IFX29(x) => {}
            Instr::IFX33(x) => {}
            Instr::IFX55(x) => {}
            Instr::IFX65(x) => {}
        }
    }
}
