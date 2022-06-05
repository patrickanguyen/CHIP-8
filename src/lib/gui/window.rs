use crate::constants::{DISPLAY_HEIGHT, DISPLAY_SCALE, DISPLAY_WIDTH, WINDOW_TITLE};
use sdl2::video::Window as SdlWindow;
use sdl2::Sdl;

pub struct Window {
    pub sdl_window: SdlWindow,
}

impl Window {
    pub fn new(sdl_context: &Sdl) -> Window {
        let video_subsystem = sdl_context.video().unwrap();
        Window {
            sdl_window: video_subsystem
                .window(
                    WINDOW_TITLE,
                    (DISPLAY_WIDTH * DISPLAY_SCALE) as u32,
                    (DISPLAY_HEIGHT * DISPLAY_SCALE) as u32,
                )
                .position_centered()
                .opengl()
                .build()
                .map_err(|e| e.to_string())
                .unwrap(),
        }
    }
}
