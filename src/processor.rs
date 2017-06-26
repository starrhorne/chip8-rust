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
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false,
            ram: [0; CHIP8_RAM],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick(&mut self, keypad: &[bool; 16]) -> OutputState {
        self.vram_changed = false;
        let opcode = self.get_opcode();
        self.run_opcode(opcode);
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
        let n = nibbles.3 as u8;

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
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
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
    fn op_6xkk(&mut self, x: usize, kk: u8) {}
    // ADD Vx, byte
    fn op_7xkk(&mut self, x: usize, kk: u8) {}
    // LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {}
    // OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {}
    // AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {}
    // XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {}
    // ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {}
    // SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {}
    // SHR Vx {, Vy}
    fn op_8xy6(&mut self, x: usize, y: usize) {}
    // SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {}
    // SHL Vx {, Vy}
    fn op_8xye(&mut self, x: usize, y: usize) {}
    // SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {}
    // LD I, addr
    fn op_annn(&mut self, nnn: usize) {}
    // JP V0, addr
    fn op_bnnn(&mut self, nnn: usize) {}
    // RND Vx, byte
    fn op_cxkk(&mut self, x: usize, kk: u8) {}
    // DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {}
    // SKP Vx
    fn op_ex9e(&mut self, x: usize) {}
    // SKNP Vx
    fn op_exa1(&mut self, x: usize) {}
    // LD Vx, DT
    fn op_fx07(&mut self, x: usize) {}
    // LD Vx, K
    fn op_fx0a(&mut self, x: usize) {}
    // LD DT, Vx
    fn op_fx15(&mut self, x: usize) {}
    // LD ST, Vx
    fn op_fx18(&mut self, x: usize) {}
    // ADD I, Vx
    fn op_fx1e(&mut self, x: usize) {}
    // LD F, Vx
    fn op_fx29(&mut self, x: usize) {}
    // LD B, Vx
    fn op_fx33(&mut self, x: usize) {}
    // LD [I], Vx
    fn op_fx55(&mut self, x: usize) {}
    // LD Vx, [I]
    fn op_fx65(&mut self, x: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let processor = Processor::new();
        assert_eq!(processor.sp, 0);
        assert_eq!(processor.stack, [0; 16]);
    }

    // CLS
    #[test]
    fn test_op_00e0() {
        let mut processor = Processor::new();
        processor.vram = [[128; CHIP8_WIDTH]; CHIP8_HEIGHT];
        processor.pc = 0x600;
        processor.run_opcode(0x00e0);
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                assert_eq!(processor.vram[y][x], 0);
            }
        }
        assert_eq!(processor.pc, 0x600 + OPCODE_SIZE);
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
        let mut processor = Processor::new();
        processor.pc = 0x400;
        processor.run_opcode(0x2666);
        assert_eq!(processor.pc, 0x0666);
        assert_eq!(processor.sp, 1);
        assert_eq!(processor.stack[0], 0x400);
    }

    // SE VX, byte
    #[test]
    fn test_op_3xkk() {
        let mut processor = Processor::new();
        processor.pc = 0x400;
        processor.v[5] = 0xfb;
        processor.run_opcode(0x35fb);
        assert_eq!(processor.pc, 0x0400 + (2 * OPCODE_SIZE));
        processor.pc = 0x400;
        processor.run_opcode(0x35fc);
        assert_eq!(processor.pc, 0x0400 + OPCODE_SIZE);
    }

    // SNE VX, byte
    #[test]
    fn test_op_4xkk() {
        let mut processor = Processor::new();
        processor.pc = 0x400;
        processor.v[5] = 0xfb;
        processor.run_opcode(0x45fc);
        assert_eq!(processor.pc, 0x0400 + (2 * OPCODE_SIZE));
        processor.pc = 0x400;
        processor.run_opcode(0x45fb);
        assert_eq!(processor.pc, 0x0400 + OPCODE_SIZE);
    }

    // SE VX, VY
    #[test]
    fn test_op_5xy0() {
        let mut processor = Processor::new();
        processor.pc = 0x400;
        processor.v[5] = 0xfb;
        processor.v[4] = 0xfb;
        processor.run_opcode(0x5540);
        assert_eq!(processor.pc, 0x0400 + (2 * OPCODE_SIZE));
        processor.pc = 0x400;
        processor.run_opcode(0x5500);
        assert_eq!(processor.pc, 0x0400 + OPCODE_SIZE);
    }



}
