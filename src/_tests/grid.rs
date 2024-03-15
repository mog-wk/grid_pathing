#![allow(unused)]

use super::*;
use crate::error::Error;

//#[test]
fn grid_build_err() {
    // FIXTURE
    let x = GridBuilder::new().build();
}
#[test]
fn grid_simple() {
    // FIXTURE
    let x = GridBuilder::new()
        .with_dimentions(12, 12)
        .build()
        .unwrap();

    // CHECK
    assert_eq!(x.dimentions(), (12, 12));
}
#[test]
fn grid_get_cell() {
    // FIXTURE
    let x = GridBuilder::new()
        .with_dimentions(5, 5)
        .build()
        .unwrap();

    // ACTION
    let aws = x.get_cell(2, 4).unwrap();

    println!("{:?}", aws);

    // CHECK
    assert_eq!(aws.grid_pos().unwrap(), (2, 4));
}
#[test]
#[should_panic]
fn grid_get_cell_fail() {
    // FIXTURE
    let x = GridBuilder::new()
        .with_dimentions(5, 5)
        .build()
        .unwrap();

    // ACTION PANIC!
    let aws = x.get_cell(12, 12).unwrap();

}
