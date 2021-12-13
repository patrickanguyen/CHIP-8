use std::collections::HashMap;

const MEMORY_SIZE: usize = 4096;
const GP_REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;
const PROGRAM_START: u16 = 0x200;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    gp_registers: [u8; GP_REGISTER_SIZE],
    program_counter: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],

    instructions: HashMap<u16, fn(&mut Chip8, u16)>,
    current_instruction: u16,

    // Timers decremented at frequency of 60 Hz (60 timers per second)
    delay_timer: u8,
    sound_timer: u8,

    keypad: [u8; KEYPAD_SIZE],
}

impl Chip8 {
    // Initialize CPU with ROM
    pub fn new(rom: &Vec<u8>) -> Chip8 {
        let mut memory = [0; MEMORY_SIZE];

        // Load fontset
        const FONT_SET: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);

        // Load ROM to program memory
        let rom_start: usize = PROGRAM_START as usize;
        let rom_end: usize = rom_start + rom.len();

        memory[rom_start..rom_end].copy_from_slice(&rom);

        // Initialize instructions hash map
        let instructions = HashMap::from([(0x00E0 as u16, Chip8::clear as fn(&mut Chip8, u16))]);

        Chip8 {
            memory: memory,
            gp_registers: [0; GP_REGISTER_SIZE],
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],

            instructions: instructions,
            current_instruction: 0,

            delay_timer: 0,
            sound_timer: 0,

            keypad: [0; KEYPAD_SIZE],
        }
    }

    // Emulate clock cycle for CHIP-8 CPI
    pub fn run_cycle(&mut self) {}

    fn clear(&mut self, opcode: u16) {}
}
