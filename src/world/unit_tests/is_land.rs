use assert2::assert;

#[allow(unused_imports)]
use super::*;

#[test]
fn is_land_on_water_returns_false() {
    /* Given */
    let (rows, cols) = (1, 1);
    let terrain = Terrain::Water;
    let expected_res = Some(false);
    let sut = World::new(terrain, rows, cols).unwrap();

    /* When */
    let res = sut.is_land(0, 0);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn is_land_on_land_returns_true() {
    /* Given */
    let (rows, cols) = (1, 1);
    let terrain = Terrain::Land;
    let expected_res = Some(true);
    let sut = World::new(terrain, rows, cols).unwrap();

    /* When */
    let res = sut.is_land(0, 0);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn is_land_out_of_bounds_returns_none() {
    /* Given */
    let (rows, cols) = (1, 1);
    let terrain = Terrain::Land;
    let expected_res = None;
    let sut = World::new(terrain, rows, cols).unwrap();

    /* When */
    let res = sut.is_land(99, 99);

    /* Then */
    assert!(res == expected_res);
}
