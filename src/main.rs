mod emulator;

use emulator::*;
use std::env;
use std::fs::File;
use std::io::Read;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WINDOW_SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut emulator = Emulator::new();

    let mut rom = File::open(&args[1]).expect("Couldn't load ROM");
    let mut rom_buffer = Vec::new();

    rom.read_to_end(&mut rom_buffer);
    emulator.load_rom(&rom_buffer);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }

        emulator.cycle();
        draw_screen(&emulator, &mut canvas)
    }
}

fn draw_screen(emu: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = emu.get_screen();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    for(i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * WINDOW_SCALE) as i32, (y * WINDOW_SCALE) as i32, WINDOW_SCALE, WINDOW_SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
