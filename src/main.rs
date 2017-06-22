extern crate sdl2;
mod drivers;
use drivers::display::DisplayDriver;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;


fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut driver = DisplayDriver::new(&sdl_context);

    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut pixels = [[0 as u8; CHIP8_WIDTH as usize]; CHIP8_HEIGHT as usize];
    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            pixels[y as usize][x as usize] = (y as u8 + x as u8) % 2;
        }
    }

    driver.draw(&pixels);

    std::thread::sleep(std::time::Duration::from_secs(4));

}

