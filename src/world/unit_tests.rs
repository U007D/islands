#![allow(clippy::unwrap_used)]

use crate::error::non_empty_rect_list_2d::Error;
use assert2::assert;
use assert_matches::assert_matches;

#[allow(unused_imports)]
use super::*;

#[test]
fn zero_row_world_does_not_construct() {
    // Given
    let (rows, cols) = (0, 0);
    let terrain = Terrain::default();
    let sut = World::new;

    // When
    let res = sut(terrain, rows, cols);

    // Then
    assert_matches!(res, Err(Error::NoData(_)));
}

#[test]
fn one_empty_row_world_does_not_construct() {
    // Given
    let (rows, cols) = (1, 0);
    let terrain = Terrain::Land;
    let sut = World::new;

    // When
    let res = sut(terrain, rows, cols);

    // Then
    assert_matches!(res, Err(Error::NoData(_)));
}

#[test]
fn one_row_world_constructs() {
    // Given
    let (rows, cols) = (1, 1);
    let terrain = Terrain::Land;
    let expected_list = NonEmptyRectList2D::new(terrain, rows, cols).unwrap();
    let (expected_rows, expected_cols) = (rows, cols);
    let sut = World::new;

    // When
    let res = sut(terrain, rows, cols);

    // Then
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.list == expected_list);
    assert!(res.rows() == expected_rows);
    assert!(res.cols() == expected_cols);
}

#[test]
fn two_row_world_upholding_row_length_invariant_constructs() {
    // Given
    let (rows, cols) = (2, 1);
    let terrain = Terrain::Water;
    let expected_list = NonEmptyRectList2D::new(terrain, rows, cols).unwrap();
    let (expected_rows, expected_cols) = (rows, cols);
    let sut = World::new;

    // When
    let res = sut(terrain, rows, cols);

    // Then
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.list == expected_list);
    assert!(res.rows() == expected_rows);
    assert!(res.cols() == expected_cols);
}
