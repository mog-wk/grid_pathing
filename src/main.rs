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
use utils::pathing;

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
        .with_dimentions(16, 12)
        .with_paddings_from(800, 600)
        .build()?;

    //grid.dev_print();

    let c1 = grid.get_cell(0, 0)?;
    let c2 = grid.get_cell(2, 0)?;
    let path = pathing::find(&grid, &c1, &c2, pathing::pitagorean_heuristic).unwrap();
    println!("{:?}", path);

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
        grid.render(&mut canvas, Color::RGB(127, 127, 127));
        canvas.set_draw_color(Color::RGB(0, 127, 0));
        for p in path.iter() {
            let x = p.0 as i32;
            let y = p.1 as i32;
            let k = 12_u32;
            canvas.fill_rect(sdl2::rect::Rect::new(
                x * grid.paddings.0 as i32 + k as i32,
                y * grid.paddings.1 as i32 + k as i32,
                grid.paddings.0 - k*2,
                grid.paddings.1 - k*2,
            ));
        }
        canvas.present();

        sleep(Duration::from_millis(60));
    }

    Ok(())
}
