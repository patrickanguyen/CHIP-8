use std::collections::HashMap;

pub struct Chip8 {
    memory: [u8; 4096],
    gp_registers: [u8; 16],
    program_counter: u16,
    stack_pointer: u16,
    stack: [u16; 16],

    instructions: HashMap<u16, fn(&mut Chip8, u16)>,
    current_instruction: u16,

    // Timers decremented at frequency of 60 Hz (60 timers per second)
    delay_timer: u8,
    sound_timer: u8,

    keypad: [u8; 16]
}

impl Chip8 {

    // Initialize CPU with ROM
    pub fn new(rom: &Vec<String>) -> Chip8 {
        let mut memory = [0; 4096];

        // Load fontset

        // Load ROM to program memory

        // Initialize instructions hash map
        let instructions = HashMap::from([
            (0x00E0 as u16, Chip8::clear as fn(&mut Chip8, u16))
        ]);


        Chip8 {
            memory: memory,
            gp_registers: [0; 16],
            program_counter: 0x200, // Program memory starts at 0x200
            stack_pointer: 0,
            stack: [0; 16],

            instructions: instructions,
            current_instruction: 0,

            delay_timer: 0,
            sound_timer: 0,

            keypad: [0; 16]
        }
    }

    // Emulate clock cycle for CHIP-8 CPI
    pub fn run_cycle(&mut self) {

    }

    fn clear(&mut self, opcode: u16) {

    }
}
