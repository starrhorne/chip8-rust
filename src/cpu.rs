use CHIP8_HEIGHT;
use CHIP8_WIDTH;

pub struct OutputState<'a> {
    pub vram: &'a [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub vram_changed: bool,
    pub beep: bool,
}


pub struct Cpu {
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT] }
    }

    pub fn tick(&self, keypad: [bool; 16]) -> OutputState {
        println!("tick");
        println!("{:?}", keypad);
        OutputState {
            vram: &self.vram,
            vram_changed: false,
            beep: false,
        }
    }
}
