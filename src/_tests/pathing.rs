use super::*;

#[test]
fn path_simple() {
    let grid = crate::utils::grid::grid()
        .with_dimentions(4, 4)
        .with_paddings_from(800, 600)
        .build()
        .unwrap();
    grid.dev_print();
}
#[test]
fn path_get_neighbors() {
    let grid = crate::utils::grid::grid()
        .with_dimentions(5, 5)
        .with_paddings_from(800, 600)
        .build()
        .unwrap();
    grid.dev_print_mov_grid();

    // top-left
    let cell = grid.get_cell(0, 0).unwrap();
    let n = get_neighbors(&grid, &cell);
    println!("cell: {:?}", cell);
    assert_eq!(n.len(), 3);

    // mid
    let cell = grid.get_cell(2, 2).unwrap();
    let n = get_neighbors(&grid, &cell);
    println!("cell: {:?}\n", cell);
    assert_eq!(n.len(), 8);

    // bot-right
    let cell = grid.get_cell(4, 4).unwrap();
    let n = get_neighbors(&grid, &cell);
    println!("cell: {:?}", cell);
    assert_eq!(n.len(), 3);
    let n_raw: Vec<Option<(usize, usize)>> = n.into_iter().map(|v| v.grid_pos()).collect();
    let aws: Vec<(usize, usize)> = vec![(3, 4), (3, 3), (4, 3)];
    for opt in n_raw {
        if let Some(raw) = opt {
            assert!(aws.contains(&raw) == true);
        }
    }
}
#[test]
fn heuristic_pitagorean() -> Result<(), Box<dyn std::error::Error>> {
    let grid = crate::utils::grid::grid()
        .with_dimentions(5, 5)
        .with_paddings_from(800, 600)
        .build()
        .unwrap();
    grid.dev_print_mov_grid();

    // FIXTURE
    let x = pitagorean_heuristic(grid.get_cell(2, 2)?, grid.get_cell(4, 2)?)?;

    // ACTION
    let aws = 320.0;

    // CHECK
    assert_eq!(x, aws);
    // FIXTURE
    let x = pitagorean_heuristic(grid.get_cell(2, 2)?, grid.get_cell(4, 3)?)?;

    // ACTION
    let aws = 341.0;

    // CHECK
    assert_eq!(x, aws);
    Ok(())
}


fn print_cell_vec(cell_vec: &Vec<&Cell>) {
    for cell in cell_vec {
        print!("({:?} ) ", cell.grid_pos());
    }
    print!("\n");
}

