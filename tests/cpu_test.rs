use chip8::cpu::Cpu;

#[test]
fn test_cls() {
    // 0x200: CLS
    const ROM: [u8; 2] = [0x00, 0xE0];
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);

    for row in cpu.display_buffer {
        for cell in row {
            assert_eq!(cell, 0);
        }
    }
}

#[test]
fn test_ret() {
    // 0x200: CALL 0x204
    // 0x202: DUMMY INSTRUCTION
    // 0x204: RET
    const ROM: [u8; 6] = [0x22, 0x04, 0x00, 0x00, 0x00, 0xEE];
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_jmp() {
    // 0x200: JMP 0x20A
    // 0x202: DUMMY INSTRUCTION
    // 0x204: DUMMY INSTRUCTION
    // 0x206: DUMMY INSTRUCTION
    // 0x208: DUMMY INSTRUCTION
    // 0x20A: DUMMY INSTRUCTION
    const ROM: [u8; 12] = [
        0x12, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    const EXPECTED_ROM: u16 = 0x20A;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_ROM);
}

#[test]
fn test_call_nnn() {
    // 0x200: CALL 0x204
    // 0x202: DUMMY INSTRUCTION
    // 0x204: DUMMY INSTRUCTION
    const ROM: [u8; 2] = [0x22, 0x04];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_se_vk_kk_true() {
    // 0x200: SE 0x5 0x0
    // 0x202: DUMMY INSTRUCTION
    // 0x204: DUMMY INSTRUCTION
    const ROM: [u8; 6] = [0x35, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_se_vk_kk_false() {
    // 0x200: SE 0x5 0x5
    // 0x202: Dummy INSTRUCTION
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x35, 0x05, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sne_vx_nn_true() {
    // 0x200: SNE 0x5 0x5
    // 0x202: Dummy INSTRUCTION
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x45, 0x05, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sne_vx_nn_false() {
    // 0x200: SNE 0x5 0x0
    // 0x202: Dummy INSTRUCTION
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x45, 0x00, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_se_vx_vy_true() {
    // 0x200: SE 0x5 0x5
    // 0x202: Dummy INSTRUCTION
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x55, 0x05, 0x00, 0x00, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_se_vx_vy_false() {
    // 0x200: LD 0x0 0xAB
    // 0x202: SE 0x0 0x50
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x60, 0xAB, 0x50, 0x50, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_ld_vx_kk() {
    // 0x200: LD 0x4 0x23
    // 0x202: DUMMY INSTRUCTION
    const ROM: [u8; 4] = [0x64, 0x23, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x4;
    const EXPECTED_VAL: u8 = 0x23;
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_add_vx_kk() {
    // 0x200: LD 0x5 0x20
    // 0x202: ADD 0x5 0x10
    // 0x204: DUMMY INSTRUCTION
    const ROM: [u8; 6] = [0x65, 0x20, 0x75, 0x10, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x5;
    const EXPECTED_VAL: u8 = 0x30;
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_ld_vx_vy() {
    // 0x200: LD 0x2 0xBC
    // 0x202: LD 0x5 0x2
    // 0x204: Dummy Instruction
    const ROM: [u8; 6] = [0x62, 0xBC, 0x85, 0x20, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x5;
    const EXPECTED_VAL: u8 = 0xBC;
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_or_vx_vy() {
    // 0x200: LD 0x1 0xF0
    // 0x202: LD 0x2 0x0F
    // 0x204: OR 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xF0, 0x62, 0x0F, 0x82, 0x11, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0xFF;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_and_vx_vy() {
    // 0x200: LD 0x1 0xAB
    // 0x202: LD 0x2 0x0D
    // 0x204: AND 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xAB, 0x62, 0x0D, 0x82, 0x12, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0x09;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_xor_vx_vy() {
    // 0x200: LD 0x1 0xAB
    // 0x202: LD 0x2 0x0D
    // 0x204: XOR 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xAB, 0x62, 0x0D, 0x82, 0x13, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0xA6;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_add_vx_vy() {
    // 0x200: LD 0x1 0xAB
    // 0x202: LD 0x2 0x0D
    // 0x204: ADD 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xAB, 0x62, 0x0D, 0x82, 0x14, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0xB8;
    const EXPECTED_F_VAL: u8 = 0x0;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_add_vx_vy_overflow() {
    // 0x200: LD 0x1 0xFE
    // 0x202: LD 0x2 0xAB
    // 0x204: ADD 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xFE, 0x62, 0xAB, 0x82, 0x14, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0xA9;
    const EXPECTED_F_VAL: u8 = 0x1;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sub_vx_vy() {
    // 0x200: LD 0x1 0xAC
    // 0x202: LD 0x2 0x12
    // 0x204: SUB 0x1 0x2
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xAC, 0x62, 0x12, 0x81, 0x25, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x1;
    const EXPECTED_VAL: u8 = 0x9A;
    const EXPECTED_F_VAL: u8 = 0x1;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sub_vx_vy_borrow() {
    // 0x200: LD 0x1 0xAC
    // 0x202: LD 0x2 0xDF
    // 0x204: SUB 0x1 0x2
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0xAC, 0x62, 0xDF, 0x81, 0x25, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x1;
    const EXPECTED_VAL: u8 = 0xCD;
    const EXPECTED_F_VAL: u8 = 0x0;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_shr_vx_vy() {
    // 0x200: LD 0x1 0xAD
    // 0x202: SHR 0x1
    // 0x204: Dummy Instruction
    const ROM: [u8; 6] = [0x61, 0xAD, 0x81, 0x26, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x1;
    const EXPECTED_VAL: u8 = 0x56;
    const EXPECTED_F_VAL: u8 = 0x1;
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_subn_vx_vy() {
    // 0x200: LD 0x1 0x1A
    // 0x202: LD 0x2 0x3C
    // 0x204: SUBN 0x1 0x2
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0x1A, 0x62, 0x3C, 0x81, 0x27, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x1;
    const EXPECTED_VAL: u8 = 0x22;
    const EXPECTED_F_VAL: u8 = 0x0;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_subn_vx_vy_borrow() {
    // 0x200: LD 0x1 0x1A
    // 0x202: LD 0x2 0x3C
    // 0x204: SUBN 0x2 0x1
    // 0x206: Dummy Instruction
    const ROM: [u8; 8] = [0x61, 0x1A, 0x62, 0x3C, 0x82, 0x17, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x2;
    const EXPECTED_VAL: u8 = 0xDE;
    const EXPECTED_F_VAL: u8 = 0x1;
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_shl_vx_vy() {
    // 0x200: LD 0x1 0xCA
    // 0x202: SHL 0x1
    // 0x204: Dummy Instruction
    const ROM: [u8; 6] = [0x61, 0xCA, 0x81, 0x2E, 0x00, 0x00];
    const EXPECTED_REG_NUM: usize = 0x1;
    const EXPECTED_VAL: u8 = 0x94;
    const EXPECTED_F_VAL: u8 = 0x1;
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.gp_reg[EXPECTED_REG_NUM], EXPECTED_VAL);
    assert_eq!(cpu.gp_reg[0xF], EXPECTED_F_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sne_vx_vy_true() {
    // 0x200: LD 0x0 0xAB
    // 0x202: SNE 0x0 0x50
    // 0x204: Dummy INSTRUCTION
    const ROM: [u8; 6] = [0x60, 0xAB, 0x90, 0x50, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_sne_vx_vy_false() {
    // 0x200: LD 0x0 0xAB
    // 0x202: LD 0x5 0xAB
    // 0x204: SNE 0x0 0x50
    const ROM: [u8; 6] = [0x60, 0xAB, 0x65, 0xAB, 0x90, 0x50];
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_ld_i_nnn() {
    // 0x200: LD I 0xDAD
    // 0x202: DUMMY INSTRUCTION
    const ROM: [u8; 4] = [0xAD, 0xAD, 0x00, 0x00];
    const EXPECTED_VAL: u16 = 0xDAD;
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.i_reg, EXPECTED_VAL);
    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn test_rnd_vx_nn() {
    // 0x200: RND 0x2 0xFF
    //0x202: Dummy Instruction

    const ROM: [u8; 4] = [0xC2, 0xFF, 0x00, 0x00];
    const EXPECTED_PC: u16 = 0x202;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
}

#[test]
fn drw_vx_vy_n() {
    // 0x200: LD I 0x006
    // 0x202: DRW 0x0 0x0 0x5
    const ROM: [u8; 4] = [0xA0, 0x06, 0xD0, 0x05];
    const EXPECTED_PC: u16 = 0x204;
    const EXPECTED_VF: u8 = 0;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
    assert_eq!(cpu.gp_reg[0xf], EXPECTED_VF);
    // TODO: ADD ASSERT FOR DISPLAY_BUFFER
}

#[test]
fn test_add_i_vx() {

}

#[test]
fn test_ld_f_vx() {
    // 0x200: LD 0x0, 0x23
    // 0x202: LD F, 0x0
    const ROM: [u8; 4] = [0x60, 0x23, 0xF0, 0x29];
    const EXPECTED_PC: u16 = 0x204;
    const EXPECTED_I_VAL: u16 = 0xAF;

    let mut cpu = Cpu::new(&ROM);
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);
    assert_eq!(cpu.i_reg, EXPECTED_I_VAL);
}

#[test]
fn test_ld_i_vx() {
    // 0x200: LD I 0x500
    // 0x202: LD 0x0 0x12
    // 0x204: LD 0x1 0x42
    // 0x206: LD 0x2 0x10
    // 0x208: LD 0x3 0x59
    // 0x20A: LD 0x4 0x8A
    // 0x20C: LD 0x5 0x4A
    // 0x20E: LD I 0x5
    const ROM: [u8; 16] = [
        0xA5, 0x00, 0x60, 0x12, 0x61, 0x42, 0x62, 0x10, 0x63, 0x59, 0x64, 0x8A, 0x65, 0x4A, 0xF6,
        0x55,
    ];
    const NUM_INSTRUCTIONS: usize = ROM.len() / 2;

    const EXPECTED_VX: usize = 5;
    const EXPECTED_MEM_VALS: [u8; 6] = [0x12, 0x42, 0x10, 0x59, 0x8A, 0x4A];
    const EXPECTED_PC: u16 = 0x210;

    let mut cpu = Cpu::new(&ROM);

    for _ in 0..NUM_INSTRUCTIONS {
        cpu.run_cycle();
    }

    assert_eq!(cpu.pc, EXPECTED_PC);

    for reg_num in 0..EXPECTED_VX + 1 {
        let index: usize = (cpu.i_reg as usize) + reg_num;
        assert_eq!(cpu.memory[index], EXPECTED_MEM_VALS[reg_num]);
    }
}

#[test]
fn test_ld_vx_i() {
    // 0x200: LD I 0x200
    // 0x202: LD 0x3, I
    const ROM: [u8; 4] = [0xA2, 0x00, 0xF3, 0x65];

    const EXPECTED_VX: usize = 3;
    const EXPECTED_REG_VALS: [u8; 4] = [0xA2, 0x00, 0xF3, 0x65];
    const EXPECTED_PC: u16 = 0x204;

    let mut cpu = Cpu::new(&ROM);

    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);

    for reg_num in 0..EXPECTED_VX + 1 {
        assert_eq!(cpu.gp_reg[reg_num], EXPECTED_REG_VALS[reg_num]);
    }
}

#[test]
fn test_ld_b_vx() {
    // 0x200: LD I 0x500
    // 0x202: LD 0x5 0xAE
    // 0x204: LD B 0x5
    const ROM: [u8; 6] = [0xA5, 0x00, 0x65, 0xAE, 0xF5, 0x33];

    const EXPECTED_I_VALS: [u8; 3] = [1, 7, 4];
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);

    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);

    for idx in 0..EXPECTED_I_VALS.len() {
        let i_idx = (cpu.i_reg as usize) + idx;
        assert_eq!(cpu.memory[i_idx], EXPECTED_I_VALS[idx]);
    }
}

#[test]
fn test_ld_b_vx_2() {
    // 0x200: LD I 0x500
    // 0x202: LD 0x5 0xAE
    // 0x204: LD B 0x5
    const ROM: [u8; 6] = [0xA5, 0x00, 0x65, 0xFF, 0xF5, 0x33];

    const EXPECTED_I_VALS: [u8; 3] = [2, 5, 5];
    const EXPECTED_PC: u16 = 0x206;

    let mut cpu = Cpu::new(&ROM);

    cpu.run_cycle();
    cpu.run_cycle();
    cpu.run_cycle();

    assert_eq!(cpu.pc, EXPECTED_PC);

    for idx in 0..EXPECTED_I_VALS.len() {
        let i_idx = (cpu.i_reg as usize) + idx;
        assert_eq!(cpu.memory[i_idx], EXPECTED_I_VALS[idx]);
    }
}
