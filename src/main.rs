#![allow(unused)]

mod error;
mod prelude;
mod utils;

use prelude::*;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

use utils::grid::grid;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;

    let video_context = sdl_context.video()?;

    let window = video_context
        .window("Doon Maze", 800, 600)
        .position(0, 0)
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;

    // dev-test
    let grid = grid()
        .with_dimentions(16, 12, true)
        .with_paddings(800, 600)
        .build()?;

    grid.dev_print();

    let mut event_pump = sdl_context.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run,
                _ => (),
            }
        }

        // logic

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        grid.render(&mut canvas,
            Color::RGB(127, 127, 127),
        );
        canvas.present();

        sleep(Duration::from_millis(60));
    }

    Ok(())
}
