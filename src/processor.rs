use CHIP8_HEIGHT;
use CHIP8_WIDTH;
use CHIP8_RAM;

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
        println!("tick: {:?}", keypad);
        self.vram_changed = false;
        self.run_opcode(self.get_opcode());
        OutputState {
            vram: &self.vram,
            vram_changed: self.vram_changed,
            beep: self.sound_timer > 0,
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }

    fn run_opcode(&self, opcode: u16) {

        let nibbles = (
            (opcode & 0x000F) as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0xF000) >> 12 as u8,
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.2 as usize;
        let y = nibbles.1 as usize;
        let n = nibbles.0 as u8;

        match nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x00, _, _, _) => self.op_0nnn(nnn),
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
    // CLS
    fn op_00e0(&self) {}
    // RET
    fn op_00ee(&self) {}
    // SYS addr
    fn op_0nnn(&self, nnn: u16) {}
    // JP addr
    fn op_1nnn(&self, nnn: u16) {}
    // CALL addr
    fn op_2nnn(&self, nnn: u16) {}
    // SE Vx, byte
    fn op_3xkk(&self, x: usize, kk: u8) {}
    // SNE Vx, byte
    fn op_4xkk(&self, x: usize, kk: u8) {}
    // SE Vx, Vy
    fn op_5xy0(&self, x: usize, y: usize) {}
    // LD Vx, byte
    fn op_6xkk(&self, x: usize, kk: u8) {}
    // ADD Vx, byte
    fn op_7xkk(&self, x: usize, kk: u8) {}
    // LD Vx, Vy
    fn op_8xy0(&self, x: usize, y: usize) {}
    // OR Vx, Vy
    fn op_8xy1(&self, x: usize, y: usize) {}
    // AND Vx, Vy
    fn op_8xy2(&self, x: usize, y: usize) {}
    // XOR Vx, Vy
    fn op_8xy3(&self, x: usize, y: usize) {}
    // ADD Vx, Vy
    fn op_8xy4(&self, x: usize, y: usize) {}
    // SUB Vx, Vy
    fn op_8xy5(&self, x: usize, y: usize) {}
    // SHR Vx {, Vy}
    fn op_8xy6(&self, x: usize, y: usize) {}
    // SUBN Vx, Vy
    fn op_8xy7(&self, x: usize, y: usize) {}
    // SHL Vx {, Vy}
    fn op_8xye(&self, x: usize, y: usize) {}
    // SNE Vx, Vy
    fn op_9xy0(&self, x: usize, y: usize) {}
    // LD I, addr
    fn op_annn(&self, nnn: u16) {}
    // JP V0, addr
    fn op_bnnn(&self, nnn: u16) {}
    // RND Vx, byte
    fn op_cxkk(&self, x: usize, kk: u8) {}
    // DRW Vx, Vy, nibble
    fn op_dxyn(&self, x: usize, y: usize, n: u8) {}
    // SKP Vx
    fn op_ex9e(&self, x: usize) {}
    // SKNP Vx
    fn op_exa1(&self, x: usize) {}
    // LD Vx, DT
    fn op_fx07(&self, x: usize) {}
    // LD Vx, K
    fn op_fx0a(&self, x: usize) {}
    // LD DT, Vx
    fn op_fx15(&self, x: usize) {}
    // LD ST, Vx
    fn op_fx18(&self, x: usize) {}
    // ADD I, Vx
    fn op_fx1e(&self, x: usize) {}
    // LD F, Vx
    fn op_fx29(&self, x: usize) {}
    // LD B, Vx
    fn op_fx33(&self, x: usize) {}
    // LD [I], Vx
    fn op_fx55(&self, x: usize) {}
    // LD Vx, [I]
    fn op_fx65(&self, x: usize) {}
}
