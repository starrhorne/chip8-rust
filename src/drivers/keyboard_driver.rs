use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std;

pub struct KeyboardDriver {
    events: sdl2::EventPump
}

impl KeyboardDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        KeyboardDriver {
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self)  {

        let mut prev_keys = HashSet::new();

        'running: loop {
            for event in self.events.poll_iter() {
                if let Event::Quit {..} = event {
                    break 'running;
                };
            }

            // Create a set of pressed Keys.
            let keys = self.events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

            // Get the difference between the new and old sets.
            let new_keys = &keys - &prev_keys;
            let old_keys = &prev_keys - &keys;

            if !new_keys.is_empty() || !old_keys.is_empty() {
                println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
            }

            prev_keys = keys;

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
