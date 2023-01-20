use super::*;
use assert2::assert;

#[test]
fn valid_get_location_returns_expected_value() {
    /* Given */
    let (rows, cols) = (1, 1);
    let value = 42;
    let expected_res = Some(&value);
    let sut = NonEmptyRectList2D::new(value, rows, cols).unwrap();

    /* When */
    let res = sut.get(0, 0);

    /* Then */
    assert!(res == expected_res);
}


#[test]
fn invalid_get_location_returns_none() {
    /* Given */
    let (rows, cols) = (1, 1);
    let value = 42;
    let expected_res = None;
    let sut = NonEmptyRectList2D::new(value, rows, cols).unwrap();

    /* When */
    let res = sut.get(99, 67);

    /* Then */
    assert!(res == expected_res);
}