extern crate sdl2;
mod drivers;
use drivers::DisplayDriver;
use drivers::AudioDriver;
use drivers::InputDriver;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;


fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut display_driver = DisplayDriver::new(&sdl_context);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);

    loop {
        match input_driver.poll() {
            Ok(v) => {
                println!("{:?}", v);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(_) => break,
        }
    }


    let mut pixels = [[0 as u8; CHIP8_WIDTH as usize]; CHIP8_HEIGHT as usize];
    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            pixels[y as usize][x as usize] = (y as u8 + x as u8) % 2;
        }
    }

    display_driver.draw(&pixels);

    audio_driver.start_beep();
    std::thread::sleep(std::time::Duration::from_secs(1));
    audio_driver.stop_beep();

    std::thread::sleep(std::time::Duration::from_secs(2));

}
