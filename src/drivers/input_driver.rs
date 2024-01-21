use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
    events: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver { events: sdl_context.event_pump().unwrap() }
    }


    pub fn poll(&mut self) -> Result<u16, ()> {

        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys: u16 = 0;
        /*
              fedc ba98 7654 3210
              8421 8421 8421 8421
            0b0000 0000 0000 0000
            
        */
        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x0002),
                Keycode::Num2 => Some(0x0004),
                Keycode::Num3 => Some(0x0008),
                Keycode::Num4 => Some(0x1000),
                Keycode::Q => Some(0x0010),
                Keycode::W => Some(0x0020),
                Keycode::E => Some(0x0040),
                Keycode::R => Some(0x2000),
                Keycode::A => Some(0x0080),
                Keycode::S => Some(0x0100),
                Keycode::D => Some(0x0200),
                Keycode::F => Some(0x4000),
                Keycode::Z => Some(0x0400),
                Keycode::X => Some(0x0001),
                Keycode::C => Some(0x0800),
                Keycode::V => Some(0x8000),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys |= i;
            }
        }

        Ok(chip8_keys)
    }
}
