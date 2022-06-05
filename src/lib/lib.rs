pub mod cpu;
pub mod gui;

mod constants;
mod state;

use cpu::Cpu;
use state::State;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {
    pub fn new(rom: &[u8]) -> Chip8 {
        Chip8 { cpu: Cpu::new(rom) }
    }

    pub fn run_cycle(&mut self) -> State {
        self.cpu.run_cycle()
    }
}
