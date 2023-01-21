use assert2::assert;

use super::*;

#[test]
fn valid_set_location_returns_expected_value() {
    // Given
    let (rows, cols) = (5, 11);
    let value = Terrain::default();
    let (row, col) = (4, 10);
    let new_value = Terrain::Land;
    let expected_res = Some(&new_value);
    let mut sut = World::new(value, rows, cols).unwrap();

    // When
    let res = sut.set(row, col, new_value);

    // Then
    assert!(res.is_some());
    let res = res.unwrap();
    assert!(res.get(row, col) == expected_res);
}

#[test]
fn invalid_set_location_returns_none() {
    // Given
    let (rows, cols) = (19, 11);
    let value = Terrain::Water;
    let expected_res = None;
    let sut = World::new(value, rows, cols).unwrap();

    // When
    let res = sut.get(99, 67);

    // Then
    assert!(res == expected_res);
}
