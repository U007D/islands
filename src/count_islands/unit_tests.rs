#![allow(clippy::unwrap_used)]

use assert2::assert;

#[allow(unused_imports)]
use super::*;
use crate::Terrain;

#[test]
fn single_cell_water_world_returns_0() {
    // Given
    let expected_res = 0;
    let world = World::new(Terrain::Water, 1, 1).unwrap();
    let sut = count_islands;

    // When
    let res = sut(&world);

    // Then
    assert!(res == expected_res);
}
