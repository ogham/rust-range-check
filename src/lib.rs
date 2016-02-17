#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::ops::Range;
use std::result;

pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;

    fn bounds(self) -> Bounds<T>;
}

impl<T> Contains<T> for Range<T>
where T: PartialOrd<T> {
    fn contains(&self, value: &T) -> bool {
        *value >= self.start && *value < self.end
    }

    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: Some(self.start),
            upper: Some(self.end),
        }
    }
}

pub trait Within<R>: Sized {
    fn is_within(&self, range: &R) -> bool;
    fn check_range(self, range: R) -> Result<Self>;
}

impl<T, R> Within<R> for T
where R: Contains<T> {
    fn is_within(&self, range: &R) -> bool {
        range.contains(self)
    }

    fn check_range(self, range: R) -> Result<Self> {
        if self.is_within(&range) {
            Ok(self)
        }
        else {
            Err(Error {
                allowed_range: range.bounds(),
                outside_value: self,
            })
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Bounds<T> {
    lower: Option<T>,
    upper: Option<T>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Error<T> {
    allowed_range: Bounds<T>,
    outside_value: T,
}

pub type Result<T> = result::Result<T, Error<T>>;


#[cfg(test)]
mod test {
    use super::{Contains, Within};

    #[test]
    fn yes() {
        assert!((1..5).contains(&3));
        assert!(3.is_within(&(1..5)));
    }

    #[test]
    fn no() {
        assert!(!(1..5).contains(&7));
        assert!(!7.is_within(&(1..5)));
    }
}

