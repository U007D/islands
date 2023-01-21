use assert2::assert;

#[allow(unused_imports)]
use super::*;

#[test]
fn rows_and_cols_accessors_return_expected_dimensions() {
    // Given
    let (rows, cols) = (91, 117);
    let value = Terrain::Land;
    let expected_res = (rows, cols);
    let sut = World::new(value, rows, cols).unwrap();

    // When
    let res = (sut.rows(), sut.cols());

    // Then
    assert!((res.0, res.1) == expected_res);
}
