use super::constants;

pub struct State {
    pub draw_flag: bool,
    pub display_buffer: [[u8; constants::DISPLAY_HEIGHT]; constants::DISPLAY_WIDTH],
}

impl State {
    pub fn new(
        draw_flag: &mut bool,
        display_buffer: [[u8; constants::DISPLAY_HEIGHT]; constants::DISPLAY_WIDTH],
    ) -> State {
        let state = State {
            draw_flag: *draw_flag,
            display_buffer,
        };

        if *draw_flag {
            *draw_flag = false;
        }

        state
    }
}
