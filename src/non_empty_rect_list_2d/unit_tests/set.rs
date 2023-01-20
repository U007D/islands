use super::*;
use assert2::assert;

#[test]
fn valid_set_location_returns_expected_value() {
    /* Given */
    let (rows, cols) = (5, 11);
    let value = 41;
    let (row, col) = (4, 10);
    let new_value = 42;
    let expected_res = Some(&new_value);
    let mut sut = NonEmptyRectList2D::new(value, rows, cols).unwrap();

    /* When */
    let res = sut.set(row, col, new_value);

    /* Then */
    assert!(res.is_some());
    let res = res.unwrap();
    assert!(res.get(row, col) == expected_res);
}


#[test]
fn invalid_set_location_returns_none() {
    /* Given */
    let (rows, cols) = (19, 11);
    let value = 42;
    let expected_res = None;
    let sut = NonEmptyRectList2D::new(value, rows, cols).unwrap();

    /* When */
    let res = sut.get(99, 67);

    /* Then */
    assert!(res == expected_res);
}