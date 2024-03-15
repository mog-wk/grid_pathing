use crate::error::Error;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{ Rect, Point };

use rand::thread_rng;

use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    dimentions: (u32, u32), // cells in width and height
    paddings: (u32, u32)
}

impl Grid {
    pub fn render(&self, canvas: &mut Canvas<Window>, line_color: Color) -> Result<(), Error> {
        let sz = canvas.output_size()?;
        let sz = (sz.0 as i32, sz.1 as i32);
        // lines
        canvas.set_draw_color(line_color);
        let (width, height) = self.dimentions;
        for j in 0..=height {
            let h = (j * self.paddings.1) as i32;
            canvas.draw_line(
                Point::new(0, h),
                Point::new(sz.0, h),
            );
        }
        for i in 0..width {
            let w = (i * self.paddings.0) as i32;
            canvas.draw_line(
                Point::new(w, 0),
                Point::new(w, sz.1),
            );
        }
        let off_set = 1;
        canvas.draw_line(
            Point::new(0, sz.1-off_set),
            Point::new(sz.0, sz.1-off_set),
        );
        canvas.draw_line(
            Point::new(sz.0-off_set, 0),
            Point::new(sz.0-off_set, sz.1),
        );

        Ok(())
    }
    pub fn dimentions(&self) -> (u32, u32) {
        self.dimentions
    }
    pub fn dev_print(&self) {
        let stdout = std::io::stdout();
        let mut handler = std::io::BufWriter::new(stdout);
        let (w, h) = self.dimentions;
        for j in 0..h {
            for i in 0..w {
                writeln!(handler, "{:?}", self.cells[j as usize][i as usize]);
            }
        }

        writeln!(handler, "dimnetions: {:?}", self.dimentions);
        writeln!(handler, "paddings: {:?}", self.paddings);
        
    }
}

pub fn grid() -> GridBuilder {
    GridBuilder::new()
}

#[derive(Debug, Default)]
pub struct GridBuilder {
    cells: Option<Vec<Vec<Cell>>>,
    dimentions: (u32, u32),
    paddings: (u32, u32),
}

impl GridBuilder {
    pub fn new() -> Self {
        Self {
            cells: None,
            dimentions: (0, 0),
            paddings: (0, 0),
        }
    }

    pub fn with_dimentions(mut self, w: i32, h: i32) -> Self {
        let mut cells = Vec::<Vec<Cell>>::with_capacity(h as usize);
        for j in 0..h {
            let mut cell_row = Vec::<Cell>::with_capacity(w as usize);
            for i in 0..w {
                cell_row.push(Cell::new((j, i), 1).unwrap());
            }
            cells.push(cell_row);
        }
        self.dimentions = (w as u32, h as u32);
        self.cells = Some(cells);
        self
    }

    pub fn with_paddings(mut self, w: u32, h: u32) -> Self {
        self.paddings = (w /self.dimentions.0, h /self.dimentions.1);
        self
        }

    pub fn build(mut self) -> Result<Grid, Error> {
        let Some(cells) = self.cells else {
            return Err(Error::GridBuildError);
        };

        Ok(Grid {
            cells,
            dimentions: self.dimentions,
            paddings: self.paddings,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    pos: (i32, i32),
    moviment_dificulty: u32,
}

impl Cell {
    fn new(pos: (i32, i32), moviment_dificulty: u32) -> Result<Self, Error> {
        if moviment_dificulty > 8 {
            return Err(Error::CellCreationError(format!("{}", moviment_dificulty)));
        }
        Ok(Self {
            pos,
            moviment_dificulty,
        })
    }
}

#[cfg(test)]
#[path = "../_tests/grid.rs"]
mod tests;
