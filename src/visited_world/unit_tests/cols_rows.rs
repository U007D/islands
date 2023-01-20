use assert2::assert;

use crate::world::Terrain;

#[allow(unused_imports)]
use super::*;

#[test]
fn rows_and_cols_accessors_return_expected_dimensions() {
    /* Given */
    let (rows, cols) = (911, 1117);
    let terrain = Terrain::Water;
    let expected_res = (rows, cols);
    let world = World::new(terrain, rows, cols).unwrap();
    let sut = VisitedWorld::from(&world);

    /* When */
    let res = (sut.rows(), sut.cols());

    /* Then */
    assert!((res.0, res.1) == expected_res);
}
