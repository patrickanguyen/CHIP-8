use super::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use super::cpu::Cpu;
use super::instructions::Instruction;

/// Return from subroutine
pub fn ret(cpu: &mut Cpu, _instr: Instruction) {
    // Pop from stack
    cpu.pc = cpu.stack[cpu.sp as usize];
    if cpu.sp > 0 {
        cpu.sp -= 1;
    }
    // Go to next instruction past the call
    cpu.pc += 2;
}

/// Jump to address NNN
pub fn jmp(cpu: &mut Cpu, instr: Instruction) {
    cpu.pc = instr.nnn;
}

/// Call subroutine at address NNN
pub fn call_nnn(cpu: &mut Cpu, instr: Instruction) {
    cpu.sp += 1;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.pc = instr.nnn;
}

/// Skip next instruction if Vx == kk
pub fn se_vx_kk(cpu: &mut Cpu, instr: Instruction) {
    if cpu.gp_reg[instr.x as usize] == instr.kk as u8 {
        cpu.pc += 4;
        return;
    }
    cpu.pc += 2;
}

/// Skip next instruction if Vx != kk
pub fn sne_vx_kk(cpu: &mut Cpu, instr: Instruction) {
    if cpu.gp_reg[instr.x as usize] != instr.kk as u8 {
        cpu.pc += 4;
        return;
    }
    cpu.pc += 2;
}

/// Skip next instruction if Vx == Vy
pub fn se_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    if cpu.gp_reg[instr.x as usize] == cpu.gp_reg[instr.y as usize] {
        cpu.pc += 4;
        return;
    }
    cpu.pc += 2;
}

/// Load value NN to register VX
pub fn ld_vx_kk(cpu: &mut Cpu, instr: Instruction) {
    cpu.gp_reg[instr.x as usize] = instr.kk as u8;
    cpu.pc += 2;
}

/// Add value kk to register VX and store result in VX
pub fn add_vx_kk(cpu: &mut Cpu, instr: Instruction) {
    let sum = cpu.gp_reg[instr.x as usize] as u16 + instr.kk;
    cpu.gp_reg[instr.x as usize] = sum as u8;
    cpu.pc += 2;
}

/// Put value of register VY to register VX
pub fn ld_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    cpu.gp_reg[instr.x as usize] = cpu.gp_reg[instr.y as usize];
    cpu.pc += 2;
}

/// Bitwise OR of VX and VY and store result in VX
pub fn or_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    cpu.gp_reg[instr.x as usize] |= cpu.gp_reg[instr.y as usize];
    cpu.pc += 2;
}

/// Bitwise AND of VX and VY and store result in VX
pub fn and_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    cpu.gp_reg[instr.x as usize] &= cpu.gp_reg[instr.y as usize];
    cpu.pc += 2;
}

/// Bitwise XOR of VX and VY and store result in VX
pub fn xor_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    cpu.gp_reg[instr.x as usize] ^= cpu.gp_reg[instr.y as usize];
    cpu.pc += 2;
}

/// Add VX and VY and store result in VX. If overflow, set VF to 1
pub fn add_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    let sum: u16 = (cpu.gp_reg[instr.x as usize] as u16) + (cpu.gp_reg[instr.y as usize] as u16);
    cpu.gp_reg[instr.x as usize] = sum as u8;

    // If overflowed, set VF to 1, 0 if not
    cpu.gp_reg[0xF] = (sum > 0xFF) as u8;

    cpu.pc += 2;
}

/// Subtract VY from VX and store difference in VX, If borrowed, set VF to 0
pub fn sub_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    let vx: i16 = cpu.gp_reg[instr.x as usize] as i16;
    let vy: i16 = cpu.gp_reg[instr.y as usize] as i16;
    let diff = vx - vy;

    cpu.gp_reg[instr.x as usize] = diff as u8;

    // If borrowed, set VF to 0, 1 if not
    cpu.gp_reg[0xF] = (vx > vy) as u8;

    cpu.pc += 2;
}

/// Shift right VX by 1, store least significant bit of VX in VF
pub fn shr_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    let vx: u8 = cpu.gp_reg[instr.x as usize];

    // Store least significant bit of VX in VF
    cpu.gp_reg[0xF] = vx & 0x01;

    // Shift right VX by 1
    cpu.gp_reg[instr.x as usize] = vx >> 1;

    cpu.pc += 2;
}

/// Subtract VY from VX and store the difference in VX. Set VF to 1 if borrow
pub fn subn_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    let vx: i16 = cpu.gp_reg[instr.x as usize] as i16;
    let vy: i16 = cpu.gp_reg[instr.y as usize] as i16;
    let diff = vy - vx;

    cpu.gp_reg[instr.x as usize] = diff as u8;

    // If borrowed, set VF to 0, 1 if not
    cpu.gp_reg[0xF] = (vx > vy) as u8;

    cpu.pc += 2;
}

/// Shift left VX by 1, store most significant bit of VX in VF
pub fn shl_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    let vx: u8 = cpu.gp_reg[instr.x as usize];

    // Store most significant bit of VX in VF
    cpu.gp_reg[0xF] = (vx >> 7) & 1;

    // Shift left VX by 1
    cpu.gp_reg[instr.x as usize] = vx << 1;

    cpu.pc += 2;
}

/// Skip next instruction if Vx != Vy
pub fn sne_vx_vy(cpu: &mut Cpu, instr: Instruction) {
    if cpu.gp_reg[instr.x as usize] != cpu.gp_reg[instr.y as usize] {
        cpu.pc += 4;
        return;
    }
    cpu.pc += 2;
}

/// Value of register I is set to NNN
pub fn ld_i_nnn(cpu: &mut Cpu, instr: Instruction) {
    cpu.i_reg = instr.nnn;
    cpu.pc += 2;
}

/// DRW VX, VY, N: Display n-byte sprite starting
/// at memory I at (VX, VY), set VF = collison
pub fn drw_vx_vy_n(cpu: &mut Cpu, instr: Instruction) {
    let x = cpu.gp_reg[instr.x as usize] as u16;
    let y = cpu.gp_reg[instr.y as usize] as u16;
    cpu.gp_reg[0xf] = 0;

    for row in 0..instr.n {
        let pixel = cpu.memory[(cpu.i_reg + row) as usize];
        for bit in 0..8 {
            // Check if the bit in pixel is set
            if pixel & (0x80 >> bit) > 0 {
                let x_idx = (x + bit) as usize % DISPLAY_WIDTH;
                let y_idx = (y + row) as usize % DISPLAY_HEIGHT;

                // Set VF to 1 if pixel in display buffer changed from 1 to 0
                if cpu.display_buffer[x_idx][y_idx] == 1 {
                    cpu.gp_reg[0xf] = 1;
                }
                cpu.display_buffer[x_idx][y_idx] ^= 1;
            }
        }
    }

    cpu.draw_flag = true;
    cpu.pc += 2;
}

/// Store registers V0 to VX (inclusive) to main memory starting at I
pub fn ld_i_vx(cpu: &mut Cpu, instr: Instruction) {
    let num_registers: usize = (instr.x + 1) as usize;

    for register in 0..num_registers {
        let mem_index: usize = (cpu.i_reg as usize) + register;
        cpu.memory[mem_index] = cpu.gp_reg[register];
    }

    cpu.pc += 2;
}

/// Load memory starting from I register to registers V0 to VX
pub fn ld_vx_i(cpu: &mut Cpu, instr: Instruction) {
    let num_registers: usize = (instr.x + 1) as usize;

    for register in 0..num_registers {
        let mem_index: usize = (cpu.i_reg as usize) + register;
        cpu.gp_reg[register] = cpu.memory[mem_index];
    }

    cpu.pc += 2;
}

pub fn ld_b_vx(cpu: &mut Cpu, instr: Instruction) {
    let vx = cpu.gp_reg[instr.x as usize];

    // Get hundreds, tens, and ones digit from VX
    let hundreds_digit: u8 = vx / 100;
    let ones_digit: u8 = vx % 10;
    let tens_digit: u8 = (vx - (hundreds_digit * 100)) / 10;

    // Store digits to memory starting at I
    cpu.memory[cpu.i_reg as usize] = hundreds_digit;
    cpu.memory[(cpu.i_reg + 1) as usize] = tens_digit;
    cpu.memory[(cpu.i_reg + 2) as usize] = ones_digit;

    cpu.pc += 2;
}
