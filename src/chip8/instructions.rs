/// Breakdown of CHIP-8 Instruction
pub struct Instruction {
    /// Raw unparsed instruction
    pub raw_instr: u16,
    /// Highest 4 bits
    pub op: u16,
    /// Lowest 12 bits
    pub nnn: u16,
    /// Lowest 4 bits
    pub n: u16,
    /// Lower 4 bits of high byte of instruction
    pub x: u16,
    /// Upper 4 bits of low byte of instruction
    pub y: u16,
    /// Lowest 8 bits of instruction
    pub kk: u16,
}

/// CHIP-8 Instruction Set
#[derive(PartialEq, Eq, Hash)]
pub enum InstructionType {
    Cls,
    Ret,
    Jmp,
    CallNnn,
    SeVxKk,
    SneVxKk,
    SeVxVy,
    LdVxKk,
    AddVxKk,
    LdVxVy,
    OrVxVy,
    AndVxVy,
    XorVxVy,
    AddVxVy,
    SubVxVy,
    ShrVxVy,
    SubnVxVy,
    ShlVxVy,
    SneVxVy,
    LdINnn,
    JmpV0Nnn,
    RndVxKk,
    DrwVxVyN,
    SkpVx,
    SkNpVx,
    LdVxDt,
    LdVxK,
    LdDtVx,
    LdStVx,
    AddIVx,
    LdFVx,
    LdBVx,
    LdIVx,
    LdVxI,
}
