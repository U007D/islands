use assert2::assert;

#[allow(unused_imports)]
use super::*;

#[test]
fn rows_and_cols_accessors_return_expected_dimensions() {
    /* Given */
    let (rows, cols) = (3, 4);
    let terrain = Terrain::Land;
    let expected_res = (rows, cols);
    let sut = World::new(terrain, rows, cols).unwrap();

    /* When */
    let res = (sut.rows(), sut.cols());

    /* Then */
    assert!((res.0, res.1) == expected_res);
}
