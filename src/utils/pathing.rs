// using A-Start pathfinding

use super::grid::*;
use crate::error::Error;
use std::collections::HashSet;

// type heuristic_type = impl Fn(&Cell, &Cell) -> Result<f64, Error>; unstable

pub fn find(
    grid: &Grid,
    start: &Cell,
    end: &Cell,
    heuristic: impl Fn(&Cell, &Cell) -> Result<f64, Error>,
) -> Option<Vec<(usize, usize)>> {
    let mut hodos = vec![start.grid_pos()?]; // hodos is the greek word for path :D
    let mut stack = vec![start];

    let end_pos = end.grid_pos()?;


    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(start.grid_pos()?);

    while stack.len() > 0 {
        let cur = stack.pop().unwrap();

        let neighbors: Vec<&Cell> = get_neighbors(grid, cur, &visited);

        if neighbors.len() == 0 {
            return None;
        }

        let mut closest = cur;
        let dist = grid.size();
        let mut dist = u32::max(dist.0, dist.1) as f64;
        println!("{:?}", cur.grid_pos()?);

        for n in neighbors.iter() {
            print!("{:?} -> ", n.grid_pos().unwrap());
        }
        print!("\n{:?}\n", visited);

        for n in neighbors.into_iter() {
            if visited.contains(&n.grid_pos()?) {
                println!("asdasd");
                continue
            }
            let h = heuristic(n, end).ok()?;
            if h < dist {
                closest = n;
                dist = h;
            }
        }
        if closest.grid_pos() == cur.grid_pos() {
            return None;
        } 
        if closest.grid_pos()? == end_pos {
            hodos.push(end_pos);
            return Some(hodos);
        }
        let pos = closest.grid_pos()?;
        visited.insert(pos);
        stack.push(closest);
        hodos.push(pos);
    }
    panic!("exit loop");
    None
}

pub fn find_step(
    grid: &Grid,
    start: &Cell,
    end: &Cell,
    heuristic: impl Fn(&Cell, &Cell) -> Result<f64, Error>,
) -> () {
}

pub fn pitagorean_heuristic(start: &Cell, end: &Cell) -> Result<f64, Error> {
    let start = start.pos();
    let end = end.pos();

    let start = (start.0 as f64, start.1 as f64);
    let end = (end.0 as f64, end.1 as f64);

    Ok(
        f64::sqrt(f64::abs(end.0 - start.0).powf(2.0) + f64::abs(end.1 - start.1).powf(2.0))
            .floor(),
    )
}

fn get_neighbors<'a>(grid: &'a Grid, cell: &'a Cell, excluded: &HashSet<(usize, usize)>) -> Vec<&'a Cell> {
    let mut neighbors = Vec::with_capacity(8);

    let id = cell.grid_pos().unwrap();
    let dim = grid.dimentions();
    let dim = (dim.0 as usize, dim.1 as usize);

    for i in 0..=2 {
        let di = match usize::checked_sub(id.0 + i, 1) {
            Some(v) => v,
            None => continue,
        };
        if di > dim.0 - 1 {
            continue;
        }
        for j in 0..=2 {
            let dj = match usize::checked_sub(id.1 + j, 1) {
                Some(v) => v,
                None => continue,
            };
            if di == id.0 && dj == id.1 {
                continue;
            }
            if dj > dim.1 - 1 {
                continue;
            }

            if !excluded.contains(&(di, dj)) {
                neighbors.push(grid.get_cell(di, dj).unwrap());
            }
        }
    }
    neighbors
}

#[cfg(test)]
#[path = "../_tests/pathing.rs"]
mod tests;
