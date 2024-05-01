#![allow(unused)]

mod error;
mod prelude;
mod utils;

use prelude::*;

use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::collections::HashSet;
use std::ops::SubAssign;
use std::thread::sleep;
use std::time::Duration;

use utils::grid::grid;
use utils::pathing;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;

    let video_context = sdl_context.video()?;

    let window = video_context
        .window("Pathfinding", 800, 600)
        .position(0, 0)
        .vulkan()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;

    // dev-test
    let grid = grid()
        .with_dimentions(16, 12)
        .with_paddings_from(800, 600)
        .build()?;

    println!(
        "{:?} {:?} {:?}",
        grid.size(),
        grid.dimentions(),
        grid.paddings
    );

    let mut path_list = Vec::<Vec<(usize, usize)>>::with_capacity(8);

    let mut current_cell = grid.get_cell(0, 0)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut mouse_buffer = HashSet::<MouseButton>::new();

    let mut mouse_delay = Duration::new(0, 0);

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
        let mouse = event_pump.mouse_state();
        let mouse_buttons = mouse.pressed_mouse_buttons().collect();
        let new_buttons = &mouse_buttons - &mouse_buffer;
        let old_buttons = &mouse_buffer - &mouse_buttons;

        if mouse_delay.is_zero() {
            for button in new_buttons.iter() {
                match button {
                    MouseButton::Left => {
                        let cell = grid.get_cell_by_pos(mouse.x(), mouse.y())?;
                        let p_path = pathing::find(
                            &grid,
                            &current_cell,
                            &cell,
                            pathing::pitagorean_heuristic,
                        );

                        match p_path {
                            Some(p) => {
                                if path_list.len() >= 8 {
                                } else {
                                    println!("{:?} {:?}", p, current_cell);
                                    path_list.push(p);
                                }
                                current_cell = cell;
                            }
                            None => (),
                        }
                        mouse_delay = std::time::Duration::from_secs(2);
                    }
                    _ => (),
                }
            }
        }
        for button in old_buttons.iter() {
            match button {
                MouseButton::Left => (),
                _ => (),
            }
        }

        mouse_buffer = new_buttons;
        mouse_delay = mouse_delay.saturating_sub(Duration::from_secs(1));

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        grid.render(&mut canvas, Color::RGB(127, 127, 127));
        let mut i = path_list.len();
        for path in path_list.iter() {
            canvas.set_draw_color(utils::path_colors::ORDER[i - 1]);
            for p in path.iter() {
                let x = p.0 as i32;
                let y = p.1 as i32;
                let k = 12_u32;
                canvas.fill_rect(sdl2::rect::Rect::new(
                    x * grid.paddings.0 as i32 + k as i32,
                    y * grid.paddings.1 as i32 + k as i32,
                    grid.paddings.0 - k * 2,
                    grid.paddings.1 - k * 2,
                ));
            }
            i -= 1;
        }
        canvas.present();

        sleep(Duration::from_millis(60));
    }

    Ok(())
}
