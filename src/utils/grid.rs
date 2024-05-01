use crate::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::Rng;

use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    size: (u32, u32),       // grid size in pizels
    dimentions: (u32, u32), // cells quantity (w, h)
    pub paddings: (u32, u32),
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
            canvas.draw_line(Point::new(0, h), Point::new(sz.0, h));
        }
        for i in 0..width {
            let w = (i * self.paddings.0) as i32;
            canvas.draw_line(Point::new(w, 0), Point::new(w, sz.1));
        }
        canvas.draw_line(Point::new(0, sz.1 - 1), Point::new(sz.0, sz.1 - 1));
        canvas.draw_line(Point::new(sz.0 - 1, 0), Point::new(sz.0 - 1, sz.1));

        // cell tranpasibility
        let off_set = 6;
        for row in self.cells.iter() {
            for cell in row {
                //let r = cell.moviment_dificulty * 20;
                let r = 12;
                let b = 12;
                let g = 12;
                canvas.set_draw_color(Color::RGB(r, g, b));
                canvas.fill_rect(Rect::new(
                    cell.pos.0 + off_set,
                    cell.pos.1 + off_set,
                    self.paddings.0 - (2 * off_set) as u32,
                    self.paddings.1 - (2 * off_set) as u32,
                ));
            }
        }

        Ok(())
    }

    pub fn dimentions(&self) -> (u32, u32) {
        self.dimentions
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get_cell(&self, i: usize, j: usize) -> Result<&Cell, Error> {
        let cell = self
            .cells
            .get(i)
            .ok_or(Error::GridCellAccessError(format!("i: {}", i)))?
            .get(j)
            .ok_or(Error::GridCellAccessError(format!("j: {}", j)))?;
        Ok(cell)
    }

    pub fn get_cell_by_pos(&self, x: i32, y: i32) -> Result<&Cell, Error> {
        let i = x / self.paddings.0 as i32;
        let j = y / self.paddings.1 as i32;
        println!(" {}, {} | {}, {}", x, y, i, j);
        let cell = self
            .cells
            .get(i as usize)
            .ok_or(Error::GridCellAccessError(format!("i: {}", i)))?
            .get(j as usize)
            .ok_or(Error::GridCellAccessError(format!("j: {}", j)))?;
        Ok(cell)
    }

    // ======== Dev Testing Functions ========
    pub fn dev_print_mov_grid(&self) {
        let stdout = std::io::stdout();
        let mut handler = std::io::BufWriter::new(stdout);

        for row in &self.cells {
            for cell in row {
                write!(
                    handler,
                    "({:?} {}), ",
                    cell.grid_pos.unwrap(),
                    cell.moviment_dificulty
                );
            }
            write!(handler, "\n");
        }
    }
    pub fn dev_print(&self) {
        let stdout = std::io::stdout();
        let mut handler = std::io::BufWriter::new(stdout);
        let (w, h) = self.dimentions;
        for j in 0..h {
            for i in 0..w {
                writeln!(handler, "{:?}", self.cells[i as usize][j as usize]);
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
    size: Option<(u32, u32)>,
    dimentions: Option<(u32, u32)>,
    paddings: Option<(u32, u32)>,
}

impl GridBuilder {
    pub fn new() -> Self {
        Self {
            cells: None,
            size: Some((0, 0)),
            dimentions: Some((0, 0)),
            paddings: Some((0, 0)),
        }
    }

    pub fn with_dimentions(mut self, w: i32, h: i32) -> Self {
        self.dimentions = Some((w as u32, h as u32));
        self
    }

    pub fn with_paddings_from(mut self, w: u32, h: u32) -> Self {
        self.paddings = Some((
            w / self.dimentions.unwrap().0,
            h / self.dimentions.unwrap().1,
        ));
        self.size = Some((w, h));
        self
    }
    pub fn init_cells(mut self, randomize: bool) -> Self {
        let Some(dimentions) = self.dimentions else {
            panic!();
        };
        let (w, h) = (dimentions.0 as i32, dimentions.1 as i32);

        let Some(paddings) = self.paddings else {
            panic!();
        };
        let (pw, ph) = (paddings.0 as i32, paddings.1 as i32);

        let mut cells = Vec::<Vec<Cell>>::with_capacity(w as usize);
        for i in 0..w {
            let mut cell_row = Vec::<Cell>::with_capacity(h as usize);
            for j in 0..h {
                if !randomize {
                    cell_row.push(
                        Cell::new((i * pw, j * ph), Some((i as usize, j as usize)), 1).unwrap(),
                    );
                } else {
                    cell_row.push(Cell::rand((i * pw, j * ph), Some((i as usize, j as usize))));
                }
            }
            cells.push(cell_row);
        }
        self.cells = Some(cells);
        self
    }

    pub fn build(mut self) -> Result<Grid, Error> {
        self = self.init_cells(true);

        Ok(Grid {
            cells: self.cells.ok_or(Error::GridBuildError)?,
            size: self.size.ok_or(Error::GridBuildError)?,
            dimentions: self.dimentions.ok_or(Error::GridBuildError)?,
            paddings: self.paddings.ok_or(Error::GridBuildError)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pos: (i32, i32),                  // abs cell pos
    grid_pos: Option<(usize, usize)>, // grid's cell pos
    moviment_dificulty: u8,
}

impl Cell {
    fn new(
        pos: (i32, i32),
        grid_pos: Option<(usize, usize)>,
        moviment_dificulty: u8,
    ) -> Result<Self, Error> {
        if moviment_dificulty > 8 {
            return Err(Error::CellCreationError(format!("{}", moviment_dificulty)));
        }
        Ok(Self {
            pos,
            grid_pos,
            moviment_dificulty,
        })
    }
    fn rand(pos: (i32, i32), grid_pos: Option<(usize, usize)>) -> Self {
        Self {
            pos,
            grid_pos,
            moviment_dificulty: rand::thread_rng().gen_range(0..=8),
        }
    }
    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }
    pub fn grid_pos(&self) -> Option<(usize, usize)> {
        self.grid_pos
    }
}

#[cfg(test)]
#[path = "../_tests/grid.rs"]
mod tests;
