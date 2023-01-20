use assert2::assert;

use crate::world::Terrain;

#[allow(unused_imports)]
use super::*;

#[test]
fn on_water_does_nothing() {
    /* Given */
    let (rows, cols) = (1, 2);
    let (unvisited_row, unvisited_col) = (0, 0);
    let (visited_row, visited_col) = (0, 1);
    let terrain = Terrain::Water;
    let expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let mut sut = VisitedWorld::from(&world);

    /* When */
    let res = sut.visit_contiguous_land(unvisited_row, unvisited_col);

    /* Then */
    assert!(res.is_unvisited_land(unvisited_row, unvisited_col) == expected_res);
    assert!(res.is_unvisited_land(visited_row, visited_col) == expected_res);
}

#[test]
fn on_1_row_2_col_land_returns_expected_results() {
    /* Given */
    let (rows, cols) = (1, 2);
    let (origin_row, origin_col) = (0, 0);
    let (visited_row, visited_col) = (0, 1);
    let terrain = Terrain::Land;
    let search_origin_expected_res = true;
    let everywhere_else_expected_res = false;
    let world = World::new(terrain, rows, cols).unwrap();
    let mut sut = VisitedWorld::from(&world);

    /* When */
    let res = sut.visit_contiguous_land(origin_row, origin_col);

    /* Then */
    assert!(res.is_unvisited_land(origin_row, origin_col) == search_origin_expected_res);
    assert!(res.is_unvisited_land(visited_row, visited_col) == everywhere_else_expected_res);
}

#[test]
fn on_2_island_world_returns_only_1_visited_island() {
    /* Given */
    let (rows, cols) = (5, 3);
    let (origin_row, origin_col) = (0, 0);
    #[rustfmt::skip]
    let expected_res = [
        false, true,  false,
        true,  true,  false,
        false, false, false,
        false, false, false,
        false, false, false,
    ].iter();
    #[rustfmt::skip]
    let terrain = vec![
        Terrain::Land,  Terrain::Land,  Terrain::Water,
        Terrain::Land,  Terrain::Land,  Terrain::Water,
        Terrain::Water, Terrain::Water, Terrain::Water,
        Terrain::Water, Terrain::Land,  Terrain::Land,
        Terrain::Water, Terrain::Land,  Terrain::Land,
    ];
    let world = (terrain, rows, cols).try_into().unwrap();
    let mut sut = VisitedWorld::from(&world);

    /* When */
    let res = sut.visit_contiguous_land(origin_row, origin_col);

    /* Then */
    assert!(res.iter().eq(expected_res));
}
