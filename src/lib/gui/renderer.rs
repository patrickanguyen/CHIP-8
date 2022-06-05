use super::window::Window;
use crate::constants::{DISPLAY_HEIGHT, DISPLAY_SCALE, DISPLAY_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub struct Renderer {
    canvas: Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Renderer {
        let sdl_window = window.sdl_window;
        Renderer {
            canvas: sdl_window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())
                .unwrap(),
        }
    }

    pub fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn update(&mut self, display_buffer: &[[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH]) {
        self.canvas.set_draw_color(Color::WHITE);

        for (x, row) in display_buffer.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * DISPLAY_SCALE) as i32,
                            (y * DISPLAY_SCALE) as i32,
                            DISPLAY_SCALE as u32,
                            DISPLAY_SCALE as u32,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
