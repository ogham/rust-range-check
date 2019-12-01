extern crate range_check;
use range_check::Check;


// These tests work with a non-number type (that is still Copy)
// to test boundary conditions.


use self::ABC::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum ABC {
    A, B, C,
}


#[test]
fn zero() {
    assert!(A.check_range(..).is_ok());
    assert!(B.check_range(..).is_ok());
    assert!(C.check_range(..).is_ok());
}

#[test]
fn one() {
    assert!(A.check_range(A .. C).is_ok());
    assert!(B.check_range(A .. C).is_ok());
    assert!(C.check_range(A .. C).is_err());
}

#[test]
fn two() {
    assert!(A.check_range(B .. C).is_err());
    assert!(B.check_range(B .. C).is_ok());
    assert!(C.check_range(B .. C).is_err());
}

#[test]
fn three() {
    assert!(A.check_range(A ..= C).is_ok());
    assert!(B.check_range(A ..= C).is_ok());
    assert!(C.check_range(A ..= C).is_ok());
}

#[test]
fn four() {
    assert!(A.check_range(B ..= B).is_err());
    assert!(B.check_range(B ..= B).is_ok());
    assert!(C.check_range(B ..= B).is_err());
}

#[test]
fn five() {
    assert!(A.check_range(A ..).is_ok());
    assert!(B.check_range(A ..).is_ok());
    assert!(C.check_range(A ..).is_ok());
}

#[test]
fn six() {
    assert!(A.check_range(B ..).is_err());
    assert!(B.check_range(B ..).is_ok());
    assert!(C.check_range(B ..).is_ok());
}

#[test]
fn seven() {
    assert!(A.check_range(.. C).is_ok());
    assert!(B.check_range(.. C).is_ok());
    assert!(C.check_range(.. C).is_err());
}

#[test]
fn eight() {
    assert!(A.check_range(.. B).is_ok());
    assert!(B.check_range(.. B).is_err());
    assert!(C.check_range(.. B).is_err());
}

#[test]
fn nine() {
    assert!(A.check_range(..= C).is_ok());
    assert!(B.check_range(..= C).is_ok());
    assert!(C.check_range(..= C).is_ok());
}

#[test]
fn ten() {
    assert!(A.check_range(..= B).is_ok());
    assert!(B.check_range(..= B).is_ok());
    assert!(C.check_range(..= B).is_err());
}
