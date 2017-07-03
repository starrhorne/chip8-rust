use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        DisplayDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}

// pub fn run() {



//     let mut lastx = 0;
//     let mut lasty = 0;

//     let mut events = sdl_context.event_pump().unwrap();

//     'main: loop {
//         for event in events.poll_iter() {

//             match event {

//                 Event::Quit {..} => break 'main,

//                 Event::KeyDown {keycode: Some(keycode), ..} => {
//                     if keycode == Keycode::Escape {
//                         break 'main
//                     } else if keycode == Keycode::Space {
//                         println!("space down");
//                         for i in 0..400 {
//                             driver.canvas.pixel(i as i16, i as i16, 0xFF000FFu32).unwrap();
//                         }
//                         driver.canvas.present();

//                     }
//                 }

//                 Event::MouseButtonDown {x, y, ..} => {
//                     let color = pixels::Color::RGB(x as u8, y as u8, 255);
//                     let _ = driver.canvas.line(lastx, lasty, x as i16, y as i16, color);
//                     let _ = driver.canvas.fill_rect(Rect::new(0,0,100,100));

//                     lastx = x as i16;
//                     lasty = y as i16;
//                     println!("mouse btn down at ({},{})", x, y);
//                     driver.canvas.present();
//                 }

//                 _ => {}
//             }
//         }
//     }
// }
