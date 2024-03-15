use crate::error::Error;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, PartialEq)]
struct Grid {
    cells: Vec<Cell>,
    dimentions: (u32, u32),
}

impl Grid {
    pub fn render(&self, canvas: &mut Canvas<Window>) -> () {
        todo!()
    }
    pub fn dimentions(&self) -> (u32, u32) {
        self.dimentions
    }
}

#[derive(Debug)]
pub struct GridBuilder {
    cells: Option<Vec<Cell>>,
    dimentions: (u32, u32),
}

impl GridBuilder {
    pub fn new() -> Self {
        Self {
            cells: None,
            dimentions: (0, 0),
        }
    }

    pub fn with_dimentions(mut self, w: i32, h: i32) -> Self {
        let sz = (w * h) as usize;
        let mut cells = Vec::<Cell>::with_capacity(sz);
        for j in 0..h {
            for i in 0..w {
                cells.push(Cell::new((i, j)));
            }
        }
        self.dimentions = (w as u32, h as u32);
        self.cells = Some(cells);
        self
    }

    pub fn build(mut self) -> Result<Grid, Error> {
        let Some(cells) = self.cells else {
            return Err(Error::GridBuildError);
        };

        Ok(Grid {
            cells,
            dimentions: self.dimentions,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    pos: (i32, i32),
    moviment_dificulty: u32,
}

impl Cell {
    fn new(pos: (i32, i32)) -> Self {
        Self {
            pos,
            moviment_dificulty: 1_u32,
        }
    }
}

#[cfg(test)]
#[path = "../_tests/grid.rs"]
mod tests;
