// using A-Start pathfinding

use super::grid::*;

pub fn find(
    grid: &Grid,
    start: &Cell,
    end: &Cell,
    heuristic: impl Fn(&Cell, &Cell) -> u32,
) -> Vec<Cell> {
    let mut path: Vec<&Cell> = vec![];
    let mut stack = vec![start];
    while stack.len() > 0 {
        let cur = stack.pop().unwrap();

        let neighbors: Vec<&Cell> = get_neighbors(grid, cur);
    }
    todo!()
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
