#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::ops::Range as RangeFromTo;
use std::result;

pub trait Contains<T>
where T: PartialOrd<T> {
    fn contains(&self, value: &T) -> bool;

    fn bounds(self) -> Bounds<T>;
}

impl<T> Contains<T> for RangeFromTo<T>
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

pub trait Within<R>: Sized + PartialOrd
where R: Contains<Self> {
    fn is_within(&self, range: &R) -> bool {
        range.contains(&self)
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
