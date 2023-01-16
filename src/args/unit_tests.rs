#![allow(clippy::unwrap_used)]

#[allow(unused_imports)]
use super::*;
use assert2::assert;

#[test]
fn attempt_to_create_args_from_no_parameters_fails() {
    /* Given */
    let expected_res = Err(Error::BadArgCount(0, 1));
    let no_args = Vec::new().into_boxed_slice();
    let sut = Args::try_from;

    /* When */
    let res = sut(no_args);

    /* Then */
    assert!(res == expected_res);
}

#[test]
fn attempt_to_create_args_from_one_parameter_succeeds() {
    /* Given */
    let one_arg = vec!["sample".into()].into_boxed_slice();
    let sut = Args::try_from;

    /* When */
    let res = sut(one_arg);

    /* Then */
    assert!(res.is_ok());
}


#[test]
fn attempt_to_create_args_from_two_parameters_fails() {
    /* Given */
    let expected_res = Err(Error::BadArgCount(2, 1));
    let two_args = vec!["one".into(), "two".into()].into_boxed_slice();
    let sut = Args::try_from;

    /* When */
    let res = sut(two_args);

    /* Then */
    assert!(res == expected_res);
}

