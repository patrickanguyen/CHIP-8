use std::collections::HashMap;

use super::constants;
use super::handlers;
use super::instructions::{Instruction, InstructionType};

pub struct Cpu {
    pub memory: [u8; constants::MEMORY_SIZE],
    pub gp_reg: [u8; constants::GP_REGISTER_SIZE],
    pub pc: u16,
    pub sp: u16,
    pub stack: [u16; constants::STACK_SIZE],

    pub i_reg: u16,

    // Timers decremented at frequency of 60 Hz (60 timers per second)
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub keypad: [u8; constants::KEYPAD_SIZE],

    pub display_buffer: [[u8; constants::DISPLAY_HEIGHT]; constants::DISPLAY_WIDTH],
    pub draw_flag: bool,

    pub instructions: HashMap<InstructionType, fn(&mut Cpu, Instruction)>,
}

impl Cpu {
    /// Initialize CPU with ROM
    pub fn new(rom: &[u8]) -> Cpu {
        let mut memory = [0; constants::MEMORY_SIZE];

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
        let rom_start: usize = constants::PROGRAM_START as usize;
        let rom_end: usize = rom_start + rom.len();

        memory[rom_start..rom_end].copy_from_slice(rom);

        Cpu {
            memory,
            gp_reg: [0; constants::GP_REGISTER_SIZE],
            pc: constants::PROGRAM_START,
            sp: 0,
            stack: [0; constants::STACK_SIZE],

            i_reg: 0,

            delay_timer: 0,
            sound_timer: 0,

            keypad: [0; constants::KEYPAD_SIZE],

            display_buffer: [[0; constants::DISPLAY_HEIGHT]; constants::DISPLAY_WIDTH],
            draw_flag: false,

            instructions: HashMap::from([
                (
                    InstructionType::Ret,
                    handlers::ret as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::Jmp,
                    handlers::jmp as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::CallNnn,
                    handlers::call_nnn as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SeVxKk,
                    handlers::se_vx_kk as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SneVxKk,
                    handlers::sne_vx_kk as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SeVxVy,
                    handlers::se_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::LdVxKk,
                    handlers::ld_vx_kk as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::AddVxKk,
                    handlers::add_vx_kk as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::LdVxVy,
                    handlers::ld_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::OrVxVy,
                    handlers::or_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::AndVxVy,
                    handlers::and_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::XorVxVy,
                    handlers::xor_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::AddVxVy,
                    handlers::add_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SubVxVy,
                    handlers::sub_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::ShrVxVy,
                    handlers::shr_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SubnVxVy,
                    handlers::subn_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::ShlVxVy,
                    handlers::shl_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::SneVxVy,
                    handlers::sne_vx_vy as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::LdINnn,
                    handlers::ld_i_nnn as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::DrwVxVyN,
                    handlers::drw_vx_vy_n as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::LdIVx,
                    handlers::ld_i_vx as fn(cpu: &mut Cpu, instr: Instruction),
                ),
                (
                    InstructionType::LdVxI,
                    handlers::ld_vx_i as fn(cpu: &mut Cpu, instr: Instruction),
                ),
            ]),
        }
    }

    /// Emulate clock cycle
    pub fn run_cycle(&mut self) {
        // Fetch
        let instr: Instruction = self.fetch();

        // Decode
        let instr_type: InstructionType = match self.decode(&instr) {
            Some(v) => v,
            None => panic!(
                "Failed to decode instruction: 0x{:X} at address 0x{:X}",
                instr.raw_instr, self.pc
            ),
        };

        // Execute
        self.execute(instr_type, instr);

        // Handle timers
    }

    /// Fetch instruction from memory
    fn fetch(&self) -> Instruction {
        let instr: u16 = ((self.memory[(self.pc as usize)] as u16) << 8)
            | (self.memory[((self.pc + 1) as usize)] as u16);

        Instruction {
            raw_instr: instr,
            op: (instr & 0xf000) >> 12,
            nnn: instr & 0xfff,
            n: instr & 0xf,
            x: (instr & 0xf00) >> 8,
            y: (instr & 0xf0) >> 4,
            kk: instr & 0xff,
        }
    }

    /// Decodes the instruction
    fn decode(&self, instr: &Instruction) -> Option<InstructionType> {
        match instr.op {
            0x0 => match instr.raw_instr {
                0x00E0 => Some(InstructionType::Cls),
                0x00EE => Some(InstructionType::Ret),
                _ => None,
            },
            0x1 => Some(InstructionType::Jmp),
            0x2 => Some(InstructionType::CallNnn),
            0x3 => Some(InstructionType::SeVxKk),
            0x4 => Some(InstructionType::SneVxKk),
            0x5 => Some(InstructionType::SeVxVy),
            0x6 => Some(InstructionType::LdVxKk),
            0x7 => Some(InstructionType::AddVxKk),
            0x8 => match instr.n {
                0x0 => Some(InstructionType::LdVxVy),
                0x1 => Some(InstructionType::OrVxVy),
                0x2 => Some(InstructionType::AndVxVy),
                0x3 => Some(InstructionType::XorVxVy),
                0x4 => Some(InstructionType::AddVxVy),
                0x5 => Some(InstructionType::SubVxVy),
                0x6 => Some(InstructionType::ShrVxVy),
                0x7 => Some(InstructionType::SubnVxVy),
                0xE => Some(InstructionType::ShlVxVy),
                _ => None,
            },
            0x9 => Some(InstructionType::SneVxVy),
            0xA => Some(InstructionType::LdINnn),
            0xB => Some(InstructionType::JmpV0Nnn),
            0xC => Some(InstructionType::RndVxKk),
            0xD => Some(InstructionType::DrwVxVyN),
            0xE => match instr.kk {
                0x9E => Some(InstructionType::SkpVx),
                0xA1 => Some(InstructionType::SkNpVx),
                _ => None,
            },
            0xF => match instr.kk {
                0x07 => Some(InstructionType::LdVxDt),
                0x0A => Some(InstructionType::LdVxK),
                0x15 => Some(InstructionType::LdDtVx),
                0x18 => Some(InstructionType::LdStVx),
                0x1E => Some(InstructionType::AddIVx),
                0x29 => Some(InstructionType::LdFVx),
                0x33 => Some(InstructionType::LdBVx),
                0x55 => Some(InstructionType::LdIVx),
                0x65 => Some(InstructionType::LdVxI),
                _ => None,
            },
            _ => None,
        }
    }

    /// Execute instruction
    fn execute(&mut self, instr_type: InstructionType, instr: Instruction) {
        match self.instructions.get(&instr_type) {
            Some(v) => v(self, instr),
            None => panic!(
                "Cannot execute instruction: 0x{:X} at 0x{:X}",
                instr.raw_instr, self.pc
            ),
        }
    }
}
