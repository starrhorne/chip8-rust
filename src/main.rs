extern crate rand;
extern crate sdl2;
mod drivers;
mod processor;
mod font;

use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::env;

use drivers::{DisplayDriver, AudioDriver, InputDriver};
use processor::Processor;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;


fn main() {
    let sleep_duration = Duration::from_millis(30);

    let sdl_context = sdl2::init().unwrap();

    let audio_driver = AudioDriver::new(&sdl_context);
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut processor = Processor::new();


    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut buffer = vec![0u8; 3584];
    f.read(&mut buffer).expect("couldn't read file");
    println!("read {} bytes", buffer.len());
    processor.load(buffer);

    loop {
        match input_driver.poll() {
            Ok(keypad) => {

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
            Err(_) => break,
        }
    }
}
