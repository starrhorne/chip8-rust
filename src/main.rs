extern crate rand;
extern crate sdl2;
mod drivers;
mod processor;
mod font;

use std::thread;
use std::time::{Duration};
use std::env;

use drivers::{DisplayDriver, AudioDriver, InputDriver, CartridgeDriver};
use processor::Processor;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;



fn main() {
    let sleep_duration = Duration::from_millis(16);
    //let sleep_duration = Duration::from_micros(16670); //60hz 
    let sdl_context = sdl2::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    let cartridge_driver = CartridgeDriver::new(cartridge_filename);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut processor = Processor::new();

    processor.load(&cartridge_driver.rom);
    let mut opcode_count = 0;
  


    while let Ok(keypad) = input_driver.poll() {
        //duct tape of the century
        let output = processor.tick(keypad);
        opcode_count+=1;

        if output.beep {
            audio_driver.start_beep();
        } 
        else {
            audio_driver.stop_beep();
        }

        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                print!("{}", if output.vram[y] >> x & 1 == 0 {" "} else {"1"});
            }
            println!();
        }

        if output.vram_changed {
            display_driver.draw(output.vram);
            processor.vram_changed = false;
        }
        

        //buffer of opcodes per 60hz, set it to where it feels right, around 10-15
        if opcode_count >=15 {
            opcode_count =0;
            if processor.sound_timer > 0 {processor.sound_timer -=1}
            if processor.delay_timer > 0 {processor.delay_timer -=1}
            thread::sleep(sleep_duration);
        }
    }
    
}
