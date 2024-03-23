// using A-Start pathfinding

use super::grid::*;
use crate::error::Error;

pub fn find(
    grid: &Grid,
    start: &Cell,
    end: &Cell,
    heuristic: impl Fn(&Cell, &Cell) -> Result<f64, Error>,
) -> Vec<Cell> {
    let mut path: Vec<&Cell> = vec![];
    let mut stack = vec![start];
    while stack.len() > 0 {
        let cur = stack.pop().unwrap();

        let neighbors: Vec<&Cell> = get_neighbors(grid, cur);
    }
    todo!()
}

pub fn pitagorean_heuristic(start: &Cell, end: &Cell) -> Result<f64, Error> {
    let start = start.pos() ;
    let end  = end.pos() ;

    let start = (start.0 as f64, start.1 as f64);
    let end = (end.0 as f64, end.1 as f64);

    Ok(f64::sqrt( f64::abs(end.0 - start.0).powf(2.0) + f64::abs(end.1 - start.1).powf(2.0) ).floor())
}

fn get_neighbors<'a>(grid: &'a Grid, cell: &'a Cell) -> Vec<&'a Cell> {

    let mut neighbors = Vec::with_capacity(8);

    let id = cell.grid_pos().unwrap();
    let dim = grid.dimentions();
    let dim = (dim.0 as usize, dim.1 as usize);

    for i in 0..=2 {
        let di = match usize::checked_sub(id.0 + i, 1) {
            Some(v) => v,
            None => continue,
        };
        if di > dim.0 -1 {
            continue
        }
        for j in 0..=2 {
            let dj = match usize::checked_sub(id.1 + j , 1) {
                Some(v) => v,
                None => continue,
            };
            if di == id.0 && dj == id.1 {
                continue
            }
            if dj > dim.1 -1 {
                continue
            }

            neighbors.push(grid.get_cell(di, dj).unwrap());
        }
    }
    neighbors
}

#[cfg(test)]
#[path = "../_tests/pathing.rs"]
mod tests;
