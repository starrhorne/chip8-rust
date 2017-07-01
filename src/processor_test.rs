use super::*;
const START_PC: usize = 0xF00;
const NEXT_PC: usize = START_PC + OPCODE_SIZE;
const SKIPPED_PC: usize = START_PC + (2 * OPCODE_SIZE);
fn build_processor() -> Processor {
    let mut processor = Processor::new();
    processor.pc = START_PC;
    processor.v = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    processor
}
#[test]
fn test_initial_state() {
    let processor = Processor::new();
    assert_eq!(processor.pc, 0x200);
    assert_eq!(processor.sp, 0);
    assert_eq!(processor.stack, [0; 16]);
    // First char in font: 0
    assert_eq!(processor.ram[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    // Last char in font: F
    assert_eq!(
        processor.ram[FONT_SET.len() - 5..FONT_SET.len()],
        [0xF0, 0x80, 0xF0, 0x80, 0x80]
    );


}
#[test]
fn test_load_data() {
    let mut processor = Processor::new();
    processor.load(vec![1, 2, 3]);
    assert_eq!(processor.ram[0x200], 1);
    assert_eq!(processor.ram[0x201], 2);
    assert_eq!(processor.ram[0x202], 3);
}

// CLS
#[test]
fn test_op_00e0() {
    let mut processor = build_processor();
    processor.vram = [[128; CHIP8_WIDTH]; CHIP8_HEIGHT];
    processor.run_opcode(0x00e0);

    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            assert_eq!(processor.vram[y][x], 0);
        }
    }
    assert_eq!(processor.pc, NEXT_PC);
}
// RET
#[test]
fn test_op_00ee() {
    let mut processor = Processor::new();
    processor.sp = 5;
    processor.stack[4] = 0x6666;
    processor.run_opcode(0x00ee);
    assert_eq!(processor.sp, 4);
    assert_eq!(processor.pc, 0x6666);
}
// JP
#[test]
fn test_op_1nnn() {
    let mut processor = Processor::new();
    processor.run_opcode(0x1666);
    assert_eq!(processor.pc, 0x0666);
}
// CALL
#[test]
fn test_op_2nnn() {
    let mut processor = build_processor();
    processor.run_opcode(0x2666);
    assert_eq!(processor.pc, 0x0666);
    assert_eq!(processor.sp, 1);
    assert_eq!(processor.stack[0], NEXT_PC);
}
// SE VX, byte
#[test]
fn test_op_3xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x3201);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x3200);
    assert_eq!(processor.pc, NEXT_PC);
}
// SNE VX, byte
#[test]
fn test_op_4xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x4200);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x4201);
    assert_eq!(processor.pc, NEXT_PC);
}
// SE VX, VY
#[test]
fn test_op_5xy0() {
    let mut processor = build_processor();
    processor.run_opcode(0x5540);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x5500);
    assert_eq!(processor.pc, NEXT_PC);
}
// LD Vx, byte
#[test]
fn test_op_6xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x65ff);
    assert_eq!(processor.v[5], 0xff);
    assert_eq!(processor.pc, NEXT_PC);
}
// ADD Vx, byte
#[test]
fn test_op_7xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x75f0);
    assert_eq!(processor.v[5], 0xf2);
    assert_eq!(processor.pc, NEXT_PC);
}
// LD Vx, Vy
#[test]
fn test_op_8xy0() {
    let mut processor = build_processor();
    processor.run_opcode(0x8050);
    assert_eq!(processor.v[0], 0x02);
    assert_eq!(processor.pc, NEXT_PC);
}
fn check_math(v1: u8, v2: u8, op: u16, result: u8, vf: u8) {
    let mut processor = build_processor();
    processor.v[0] = v1;
    processor.v[1] = v2;
    processor.v[0x0f] = 0;
    processor.run_opcode(0x8010 + op);
    assert_eq!(processor.v[0], result);
    assert_eq!(processor.v[0x0f], vf);
    assert_eq!(processor.pc, NEXT_PC);
}
// OR Vx, Vy
#[test]
fn test_op_8xy1() {
    // 0x0F or 0xF0 == 0xFF
    check_math(0x0F, 0xF0, 1, 0xFF, 0);
}
// AND Vx, Vy
#[test]
fn test_op_8xy2() {
    // 0x0F and 0xFF == 0x0F
    check_math(0x0F, 0xFF, 2, 0x0F, 0);
}
// XOR Vx, Vy
#[test]
fn test_op_8xy3() {
    // 0x0F xor 0xFF == 0xF0
    check_math(0x0F, 0xFF, 3, 0xF0, 0);
}
// ADD Vx, Vy
#[test]
fn test_op_8xy4() {
    check_math(0x0F, 0x0F, 4, 0x1E, 0);
    check_math(0xFF, 0xFF, 4, 0xFE, 1);
}
// SUB Vx, Vy
#[test]
fn test_op_8xy5() {
    check_math(0x0F, 0x01, 5, 0x0E, 1);
    check_math(0x0F, 0xFF, 5, 0x10, 0);
}
// SHR Vx
#[test]
fn test_op_8x06() {
    // 4 >> 1 == 2
    check_math(0x04, 0, 6, 0x02, 0);
    // 5 >> 1 == 2 with carry
    check_math(0x05, 0, 6, 0x02, 1);
}
// SUBN Vx, Vy
#[test]
fn test_op_8xy7() {
    check_math(0x01, 0x0F, 7, 0x0E, 1);
    check_math(0xFF, 0x0F, 7, 0x10, 0);
}

// SHL Vx
#[test]
fn test_op_8x0e() {
    check_math(0b11000000, 0, 0x0e, 0b10000000, 1);
    check_math(0b00000111, 0, 0x0e, 0b00001110, 0);
}

// SNE VX, VY
#[test]
fn test_op_9xy0() {
    let mut processor = build_processor();
    processor.run_opcode(0x90e0);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x9010);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD I, byte
#[test]
fn test_op_annn() {
    let mut processor = build_processor();
    processor.run_opcode(0xa123);
    assert_eq!(processor.i, 0x123);
}

// JP V0, addr
#[test]
fn test_op_bnnn() {
    let mut processor = build_processor();
    processor.v[0] = 3;
    processor.run_opcode(0xb123);
    assert_eq!(processor.pc, 0x126);
}

// RND Vx, byte
// Generates random u8, then ANDs it with kk.
// We can't test randomness, but we can test the AND.
#[test]
fn test_op_cxkk() {
    let mut processor = build_processor();
    processor.run_opcode(0xc000);
    assert_eq!(processor.v[0], 0);
    processor.run_opcode(0xc00f);
    assert_eq!(processor.v[0] & 0xf0, 0);
}

// DRW Vx, Vy, nibble
#[test]
fn test_op_dxyn() {
    let mut processor = build_processor();
    processor.i = 0;
    processor.ram[0] = 0b11111111;
    processor.ram[1] = 0b00000000;
    processor.vram[0][0] = 1;
    processor.vram[0][1] = 0;
    processor.vram[1][0] = 1;
    processor.vram[1][1] = 0;
    processor.v[0] = 0;
    processor.run_opcode(0xd002);

    assert_eq!(processor.vram[0][0], 0);
    assert_eq!(processor.vram[0][1], 1);
    assert_eq!(processor.vram[1][0], 1);
    assert_eq!(processor.vram[1][1], 0);
    assert_eq!(processor.v[0x0f], 1);
    assert!(processor.vram_changed);
    assert_eq!(processor.pc, NEXT_PC);
}


#[test]
fn test_op_dxyn_wrap_horizontal() {
    let mut processor = build_processor();

    let x = CHIP8_WIDTH - 4;

    processor.i = 0;
    processor.ram[0] = 0b11111111;
    processor.v[0] = x as u8;
    processor.v[1] = 0;
    processor.run_opcode(0xd011);

    assert_eq!(processor.vram[0][x - 1], 0);
    assert_eq!(processor.vram[0][x], 1);
    assert_eq!(processor.vram[0][x + 1], 1);
    assert_eq!(processor.vram[0][x + 2], 1);
    assert_eq!(processor.vram[0][x + 3], 1);
    assert_eq!(processor.vram[0][0], 1);
    assert_eq!(processor.vram[0][1], 1);
    assert_eq!(processor.vram[0][2], 1);
    assert_eq!(processor.vram[0][3], 1);
    assert_eq!(processor.vram[0][4], 0);

    assert_eq!(processor.v[0x0f], 0);
}

// DRW Vx, Vy, nibble
#[test]
fn test_op_dxyn_wrap_vertical() {
    let mut processor = build_processor();
    let y = CHIP8_HEIGHT - 1;

    processor.i = 0;
    processor.ram[0] = 0b11111111;
    processor.ram[1] = 0b11111111;
    processor.v[0] = 0;
    processor.v[1] = y as u8;
    processor.run_opcode(0xd012);

    assert_eq!(processor.vram[y][0], 1);
    assert_eq!(processor.vram[0][0], 1);
    assert_eq!(processor.v[0x0f], 0);
}


// SKP Vx
#[test]
fn test_op_ex9e() {
    let mut processor = build_processor();
    processor.keypad[9] = true;
    processor.v[5] = 9;
    processor.run_opcode(0xe59e);
    assert_eq!(processor.pc, SKIPPED_PC);


    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.run_opcode(0xe59e);
    assert_eq!(processor.pc, NEXT_PC);
}

// SKNP Vx
#[test]
fn test_op_exa1() {
    let mut processor = build_processor();
    processor.keypad[9] = true;
    processor.v[5] = 9;
    processor.run_opcode(0xe5a1);
    assert_eq!(processor.pc, NEXT_PC);


    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.run_opcode(0xe5a1);
    assert_eq!(processor.pc, SKIPPED_PC);
}

// LD Vx, DT
#[test]
fn test_op_fx07() {
    let mut processor = build_processor();
    processor.delay_timer = 20;
    processor.run_opcode(0xf507);
    assert_eq!(processor.v[5], 20);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, K
#[test]
fn test_op_fx0a() {
    let mut processor = build_processor();
    processor.run_opcode(0xf50a);
    assert_eq!(processor.keypad_waiting, true);
    assert_eq!(processor.keypad_register, 5);
    assert_eq!(processor.pc, NEXT_PC);

    // Tick with no keypresses doesn't do anything
    processor.tick([false; 16]);
    assert_eq!(processor.keypad_waiting, true);
    assert_eq!(processor.keypad_register, 5);
    assert_eq!(processor.pc, NEXT_PC);

    // Tick with a keypress finishes wait and loads
    // first pressed key into vx
    processor.tick([true; 16]);
    assert_eq!(processor.keypad_waiting, false);
    assert_eq!(processor.v[5], 0);
    assert_eq!(processor.pc, NEXT_PC);


}

// LD DT, vX
#[test]
fn test_op_fx15() {
    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.run_opcode(0xf515);
    assert_eq!(processor.delay_timer, 9);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD ST, vX
#[test]
fn test_op_fx18() {
    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.run_opcode(0xf518);
    assert_eq!(processor.sound_timer, 9);
    assert_eq!(processor.pc, NEXT_PC);
}

// ADD I, Vx
#[test]
fn test_op_fx1e() {
    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.i = 9;
    processor.run_opcode(0xf51e);
    assert_eq!(processor.i, 18);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD F, Vx
#[test]
fn test_op_fx29() {
    let mut processor = build_processor();
    processor.v[5] = 9;
    processor.run_opcode(0xf529);
    assert_eq!(processor.i, 5 * 9);
    assert_eq!(processor.pc, NEXT_PC);

}

// LD B, Vx
#[test]
fn test_op_fx33() {
    let mut processor = build_processor();
    processor.v[5] = 123;
    processor.i = 1000;
    processor.run_opcode(0xf533);
    assert_eq!(processor.ram[1000], 1);
    assert_eq!(processor.ram[1001], 2);
    assert_eq!(processor.ram[1002], 3);
    assert_eq!(processor.pc, NEXT_PC);

}

// LD [I], Vx
#[test]
fn test_op_fx55() {
    let mut processor = build_processor();
    processor.i = 1000;
    processor.run_opcode(0xff55);
    for i in 0..16 {
        assert_eq!(processor.ram[1000 + i as usize], processor.v[i]);
    }
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, [I]
#[test]
fn test_op_fx65() {
    let mut processor = build_processor();
    for i in 0..16 as usize {
        processor.ram[1000 + i] = i as u8;
    }
    processor.i = 1000;
    processor.run_opcode(0xff65);

    for i in 0..16 as usize {
        assert_eq!(processor.v[i], processor.ram[1000 + i]);
    }
    assert_eq!(processor.pc, NEXT_PC);

}

#[test]
fn test_timers() {
    let mut processor = build_processor();
    processor.delay_timer = 200;
    processor.sound_timer = 100;
    processor.tick([false; 16]);
    assert_eq!(processor.delay_timer, 199);
    assert_eq!(processor.sound_timer, 99);
}
