extern crate sdl2;

use std::env::args;
use std::fs;
use std::process;

use chip8::gui::{events::Events, renderer::Renderer, window::Window};
use chip8::Chip8;

fn read_rom(args: Vec<String>) -> Result<Vec<u8>, String> {
    if args.len() != 2 {
        return Err(String::from("Invalid amount of arguments"));
    }

    let rom_path = &args[1];
    let rom = fs::read(rom_path);

    match rom {
        Ok(rom) => Ok(rom),
        Err(err) => Err(err.to_string()),
    }
}

pub fn main() {
    let args: Vec<String> = args().collect();

    let rom: Vec<u8> = read_rom(args).unwrap_or_else(|err| {
        println!("Error reading ROM: {}", err);
        process::exit(1);
    });

    let sdl_context = sdl2::init().unwrap();
    let window = Window::new(&sdl_context);

    let mut chip8 = Chip8::new(&rom);
    let mut renderer = Renderer::new(window);
    let mut events = Events::new(&sdl_context);

    renderer.clear_screen();
    events.handle_events(&mut chip8, &mut renderer);
}
