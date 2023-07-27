extern crate rand;
extern crate sdl2;
mod drivers;
mod processor;
mod font;

use std::thread;
use std::time::Duration;
use std::env;

use drivers::{DisplayDriver, AudioDriver, InputDriver, CartridgeDriver};
use processor::Processor;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;



fn main() {
    //let sleep_duration = Duration::from_micros(16670); //60hz
    let sleep_duration = Duration::from_millis(2); //500hz

    let sdl_context = sdl2::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    let cartridge_driver = CartridgeDriver::new(cartridge_filename);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut processor = Processor::new();

    processor.load(&cartridge_driver.rom);

    while let Ok(keypad) = input_driver.poll() {

        let output = processor.tick(keypad);

        if output.vram_changed {
            display_driver.draw(output.vram);
        }

        if output.beep {
            audio_driver.start_beep();
        } else {
            audio_driver.stop_beep();
        }

        thread::sleep(sleep_duration);
    }
}
