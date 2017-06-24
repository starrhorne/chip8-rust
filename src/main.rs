extern crate sdl2;
mod drivers;
mod cpu;

use drivers::DisplayDriver;
use drivers::AudioDriver;
use drivers::InputDriver;

use cpu::Cpu;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;


fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut display_driver = DisplayDriver::new(&sdl_context);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);

    let cpu = Cpu::new();

    loop {
        match input_driver.poll() {
            Ok(keypad) => {
                let output = cpu.tick(keypad);
                if output.vram_changed {
                    display_driver.draw(output.vram);
                }
                if output.beep {
                    audio_driver.start_beep();
                } else {
                    audio_driver.stop_beep();
                }

                std::thread::sleep(std::time::Duration::from_millis(500));

            }
            Err(_) => break,
        }
    }
}
