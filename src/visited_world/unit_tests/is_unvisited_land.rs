use assert2::assert;

use crate::world::Terrain;

#[allow(unused_imports)]
use super::*;

#[test]
fn water_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let terrain = Terrain::Water;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = VisitedWorld::from(&world);

    /* When */
    let res = sut.is_unvisited_land(row, col);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn visited_land_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let terrain = Terrain::Land;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = {
        let mut tmp = VisitedWorld::from(&world);
        tmp.set_visited(row, col, true);
        tmp
    };

    /* When */
    let res = sut.is_unvisited_land(row, col);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn out_of_bounds_row_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let (oob_row, _oob_col) = (92, 61);
    let terrain = Terrain::Land;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = {
        let mut tmp = VisitedWorld::from(&world);
        tmp.set_visited(row, col, true);
        tmp
    };

    /* When */
    let res = sut.is_unvisited_land(oob_row, col);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn out_of_bounds_col_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let (_oob_row, oob_col) = (92, 61);
    let terrain = Terrain::Land;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = {
        let mut tmp = VisitedWorld::from(&world);
        tmp.set_visited(row, col, true);
        tmp
    };

    /* When */
    let res = sut.is_unvisited_land(row, oob_col);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn out_of_bounds_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let (oob_row, oob_col) = (92, 61);
    let terrain = Terrain::Land;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = {
        let mut tmp = VisitedWorld::from(&world);
        tmp.set_visited(row, col, true);
        tmp
    };

    /* When */
    let res = sut.is_unvisited_land(oob_row, oob_col);

    /* Then */
    assert!(res == expected_res);
}


#[test]
fn unvisited_land_location_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let (row, col) = (0, 0);
    let terrain = Terrain::Land;
    let expected_res = true;
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = VisitedWorld::from(&world);

    /* When */
    let res = sut.is_unvisited_land(row, col);

    /* Then */
    assert!(res == expected_res);
}
