use rand;
use rand::Rng;
use font::FONT_SET;

use CHIP8_HEIGHT;
use CHIP8_WIDTH;
use CHIP8_RAM;

const OPCODE_SIZE: usize = 2;

pub struct OutputState<'a> {
    pub vram: &'a [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub vram_changed: bool,
    pub beep: bool,
}

pub struct Processor {
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    vram_changed: bool,
    ram: [u8; CHIP8_RAM],
    stack: [usize; 16],
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    keypad_waiting: bool,
    keypad_register: usize,
}

impl Processor {
    pub fn new() -> Self {

        let mut ram = [0u8; CHIP8_RAM];
        for i in 0..FONT_SET.len() {
            ram[i] = FONT_SET[i];
        }

        Processor {
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false,
            ram: ram,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            keypad_waiting: false,
            keypad_register: 0,
        }
    }

    pub fn tick(&mut self, keypad: [bool; 16]) -> OutputState {
        self.keypad = keypad;
        self.vram_changed = false;

        if self.keypad_waiting {
            for i in 0..keypad.len() {
                if keypad[i] {
                    self.keypad_waiting = false;
                    self.v[self.keypad_register] = i as u8;
                    break;
                }
            }
        } else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1
            }
            let opcode = self.get_opcode();
            self.run_opcode(opcode);
        }

        OutputState {
            vram: &self.vram,
            vram_changed: self.vram_changed,
            beep: self.sound_timer > 0,
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => return,
        }
    }
    // CLS: Clear the display.
    fn op_00e0(&mut self) {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                self.vram[y][x] = 0;
            }
        }
        self.vram_changed = true;
        self.pc += OPCODE_SIZE;
    }
    // RET:  Return from a subroutine.
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }
    // JP addr
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    // CALL addr
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }
    // SE Vx, byte:  Skip next instruction if Vx = kk.
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        self.pc += OPCODE_SIZE * (if self.v[x] == kk { 2 } else { 1 });
    }
    // SNE Vx, byte. Skip next instruction if Vx != kk.
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        self.pc += OPCODE_SIZE * (if self.v[x] != kk { 2 } else { 1 });
    }
    // SE Vx, Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        self.pc += OPCODE_SIZE * (if self.v[x] == self.v[y] { 2 } else { 1 });
    }
    // LD Vx, byte
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc += OPCODE_SIZE;
    }
    // ADD Vx, byte
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
        self.pc += OPCODE_SIZE;
    }
    // LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += OPCODE_SIZE;
    }
    // OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += OPCODE_SIZE;
    }
    // AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += OPCODE_SIZE;
    }
    // XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += OPCODE_SIZE;
    }
    // ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
        self.pc += OPCODE_SIZE;
    }
    // SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += OPCODE_SIZE;
    }
    // SHR Vx {, Vy}
    fn op_8x06(&mut self, x: usize) {
        self.v[0x0f] = self.v[x] & 0x01;
        self.v[x] = self.v[x] >> 1;
        self.pc += OPCODE_SIZE;
    }
    // SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += OPCODE_SIZE;
    }
    // SHL Vx {, Vy}
    fn op_8x0e(&mut self, x: usize) {
        self.v[0x0f] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] = self.v[x] << 1;
        self.pc += OPCODE_SIZE;
    }


    // SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        self.pc += OPCODE_SIZE * (if self.v[x] != self.v[y] { 2 } else { 1 });
    }

    // LD I, addr
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
        self.pc += OPCODE_SIZE;
    }

    // JP V0, addr
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc = (self.v[0] as usize) + nnn;
    }

    // RND Vx, byte
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        self.pc += OPCODE_SIZE;
    }

    // DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0x0f] = 0;
        for byte in self.i..(self.i + n) {
            let y = (self.v[y] as usize + byte) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % CHIP8_WIDTH;
                let color = (self.ram[byte] >> bit) & 1;
                self.v[0x0f] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;

            }
        }
        self.vram_changed = true;
        self.pc += OPCODE_SIZE;
    }

    // SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        let pressed = self.keypad[self.v[x] as usize];
        self.pc += OPCODE_SIZE * (if pressed { 2 } else { 1 });
    }

    // SKNP Vx
    fn op_exa1(&mut self, x: usize) {
        let pressed = self.keypad[self.v[x] as usize];
        self.pc += OPCODE_SIZE * (if !pressed { 2 } else { 1 });
    }
    // LD Vx, DT
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += OPCODE_SIZE;
    }
    // LD Vx, K
    fn op_fx0a(&mut self, x: usize) {
        self.keypad_waiting = true;
        self.keypad_register = x;
        self.pc += OPCODE_SIZE;
    }

    // LD DT, Vx
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += OPCODE_SIZE;
    }
    // LD ST, Vx
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += OPCODE_SIZE;
    }
    // ADD I, Vx
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.pc += OPCODE_SIZE;
    }
    // LD F, Vx
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
        self.pc += OPCODE_SIZE;
    }

    // LD B, Vx
    fn op_fx33(&mut self, x: usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        self.pc += OPCODE_SIZE;
    }

    // LD [I], Vx
    fn op_fx55(&mut self, x: usize) {
        for i in 0..self.v[x] as usize {
            self.ram[self.i + i] = self.v[i];
        }
        self.pc += OPCODE_SIZE;
    }

    // LD Vx, [I]
    fn op_fx65(&mut self, x: usize) {
        for i in 0..self.v[x] as usize {
            self.v[i] = self.ram[self.i + i];
        }
        self.pc += OPCODE_SIZE;
    }
}
#[cfg(test)]
mod tests {
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
        assert_eq!(processor.stack[0], START_PC);
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
        processor.v[5] = 0x0f;
        processor.run_opcode(0xf555);
        for i in 0..0x0f {
            assert_eq!(processor.ram[1000 + i as usize], processor.v[i]);
        }
        assert_eq!(processor.pc, NEXT_PC);
    }

    // LD Vx, [I]
    #[test]
    fn test_op_fx65() {
        let mut processor = build_processor();
        for i in 0..0x0f as usize {
            processor.ram[1000 + i] = i as u8;
        }
        processor.i = 1000;
        processor.v[5] = 0x0f;
        processor.run_opcode(0xf565);

        for i in 0..0x0f as usize {
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
}
