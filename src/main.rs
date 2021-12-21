extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

use std::fs;

use chip8_lib::chip8::cpu::Cpu;

pub fn main() -> Result<(), String> {
    let rom_path = "roms/test_opcode.ch8";
    let rom: Vec<u8> = fs::read(rom_path).expect("Failed to open ROM");

    let mut cpu = Cpu::new(&rom);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP-8 Emulator", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // TODO: REMOVE IF STATEMENT
        // DEBUG STATEMENT
        if !cpu.draw_flag {
            cpu.run_cycle();
        }

        if cpu.draw_flag {
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            for x in 0..cpu.display_buffer.len() {
                for y in 0..cpu.display_buffer[x].len() {
                    if cpu.display_buffer[x][y] == 1 {
                        canvas.draw_point(Point::new(x as i32, y as i32))?;
                    }
                }
            }

            canvas.present();
            // cpu.draw_flag = false;
        }

        ::std::thread::sleep(Duration::new(0, 50_000_000u32 / 3));
        // The rest of the game loop goes here...
    }

    Ok(())
}
