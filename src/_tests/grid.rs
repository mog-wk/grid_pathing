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
