use chip8_lib::chip8::cpu::Cpu;

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
