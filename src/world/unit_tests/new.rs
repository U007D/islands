use std::iter;

use assert2::assert;

use super::*;

#[test]
fn zero_row_one_col_world_is_empty() {
    // Given
    let (rows, cols) = (0, 1);
    let value = Terrain::Water;
    let sut = World::new;

    // When
    let res = sut(value, rows, cols);

    // Then
    let res = res.unwrap();
    assert!(res.is_empty());
}

#[test]
fn one_row_zero_col_world_is_empty() {
    // Given
    let (rows, cols) = (1, 0);
    let value = Terrain::default();
    let sut = World::new;

    // When
    let res = sut(value, rows, cols);

    // Then
    let res = res.unwrap();
    assert!(res.is_empty());
}

#[test]
fn zero_row_zero_col_world_is_empty() {
    // Given
    let (rows, cols) = (0, 0);
    let value = Terrain::Land;
    let sut = World::new;

    // When
    let res = sut(value, rows, cols);

    // Then
    let res = res.unwrap();
    assert!(res.is_empty());
}

#[test]
fn one_row_one_col_world_constructs() {
    // Given
    let (rows, cols) = (1, 1);
    let value = Terrain::Water;
    let expected_res = iter::once(Terrain::Water);
    let sut = World::new;

    // When
    let res = sut(value, rows, cols);

    // Then
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.into_iter().eq(expected_res));
}

#[test]
fn two_row_three_col_world_constructs() {
    // Given
    let (rows, cols) = (2, 3);
    let value = Terrain::default();
    let expected_res = vec![Terrain::Water; rows * cols];
    let sut = World::new;

    // When
    let res = sut(value, rows, cols);

    // Then
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.into_iter().eq(expected_res));
}

#[test]
fn rows_and_cols_accessors_return_expected_dimensions() {
    // Given
    let (rows, cols) = (3, 4);
    let value = Terrain::Land;
    let expected_res = (rows, cols);
    let sut = World::new(value, rows, cols).unwrap();

    // When
    let res = (sut.rows(), sut.cols());

    // Then
    assert!((res.0, res.1) == expected_res);
}
