use super::*;
use assert2::assert;
use assert_matches::assert_matches;
use std::iter;

#[test]
fn zero_row_one_col_non_empty_rect_list_2d_fails_to_construct() {
    /* Given */
    let (rows, cols) = (0, 1);
    let value = 42;
    let sut = NonEmptyRectList2D::new;

    /* When */
    let res = sut(value, rows, cols);

    /* Then */
    assert_matches!(res, Err(Error::NoData(_)));
}

#[test]
fn one_row_zero_col_non_empty_rect_list_2d_fails_to_construct() {
    /* Given */
    let (rows, cols) = (1, 0);
    let value = 42;
    let sut = NonEmptyRectList2D::new;

    /* When */
    let res = sut(value, rows, cols);

    /* Then */
    assert_matches!(res, Err(Error::NoData(_)));
}

#[test]
fn zero_row_zero_col_non_empty_rect_list_2d_fails_to_construct() {
    /* Given */
    let (rows, cols) = (0, 0);
    let value = 42;
    let sut = NonEmptyRectList2D::new;

    /* When */
    let res = sut(value, rows, cols);

    /* Then */
    assert_matches!(res, Err(Error::NoData(_)));
}

#[test]
fn one_row_one_col_non_empty_rect_list_2d_constructs() {
    /* Given */
    let (rows, cols) = (1, 1);
    let value = true;
    let expected_result = iter::once(true);
    let sut = NonEmptyRectList2D::new;

    /* When */
    let res = sut(value, rows, cols);

    /* Then */
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.into_iter().eq(expected_result));
}

#[test]
fn two_row_three_col_non_empty_rect_list_2d_constructs() {
    /* Given */
    let (rows, cols) = (2, 3);
    let value = 'X';
    #[rustfmt::skip]
        let expected_result = [
        'X', 'X', 'X',
        'X', 'X', 'X',
    ];
    let sut = NonEmptyRectList2D::new;

    /* When */
    let res = sut(value, rows, cols);

    /* Then */
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.into_iter().eq(expected_result));
}

#[test]
fn rows_and_cols_accessors_return_expected_dimensions() {
    /* Given */
    let (rows, cols) = (3, 4);
    let value = false;
    let expected_res = (rows, cols);
    let sut = NonEmptyRectList2D::new(value, rows, cols).unwrap();

    /* When */
    let res = (sut.rows(), sut.cols());

    /* Then */
    assert!((res.0, res.1) == expected_res);
}
