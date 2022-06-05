use super::renderer::Renderer;
use crate::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::Sdl;
use std::thread::sleep;
use std::time::Duration;

pub struct Events {
    event_pump: EventPump,
}

impl Events {
    pub fn new(sdl_context: &Sdl) -> Events {
        Events {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn handle_events(&mut self, chip8: &mut Chip8, renderer: &mut Renderer) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            let state = chip8.run_cycle();

            if state.draw_flag {
                renderer.update(&state.display_buffer);
            }

            sleep(Duration::new(0, 16_666_666));
        }
    }
}
