#![allow(unused)]

mod utils;
mod prelude;
mod error;

use prelude::*;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::time::Duration;
use std::thread::sleep;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;

    let video_context = sdl_context.video()?;

    let window = video_context
        .window("Doon Maze", 800, 600)
        .position(0, 0)
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;

    let mut event_pump = sdl_context.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => println!("{:?}", event),
            }
        }

        // logic


        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        sleep(Duration::from_millis(60));

    }

    Ok(())
}
