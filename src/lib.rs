#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::any::Any;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo};
use std::result;


pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;

    fn bounds(self) -> Bounds<T>;
}

impl<T> Contains<T> for Range<T>
where T: PartialOrd {
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

impl<T> Contains<T> for RangeFrom<T>
where T: PartialOrd {
    fn contains(&self, value: &T) -> bool {
        *value >= self.start
    }

    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: Some(self.start),
            upper: None,
        }
    }
}

impl<T> Contains<T> for RangeTo<T>
where T: PartialOrd {
    fn contains(&self, value: &T) -> bool {
        *value < self.end
    }

    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: None,
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

impl<T: fmt::Debug> fmt::Display for Bounds<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref lower) = self.lower {
            try!(write!(f, "{:?}", lower));
        }

        try!(write!(f, " .. "));

        if let Some(ref upper) = self.upper {
            try!(write!(f, "{:?}", upper));
        }

        Ok(())
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct Error<T> {
    allowed_range: Bounds<T>,
    outside_value: T,
}

impl<T: fmt::Debug + Any> ErrorTrait for Error<T> {
    fn description(&self) -> &str {
        "value outside of range"
    }
}

impl<T: fmt::Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value ({:?}) outside of range ({})",
            self.outside_value, self.allowed_range)
    }
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

    #[test]
    fn from_yes() {
        assert!((1..).contains(&3));
        assert!(3.is_within(&(1..)));
    }

    #[test]
    fn from_no() {
        assert!(!(1..).contains(&-7));
        assert!(!(-7).is_within(&(1..)));
    }

    #[test]
    fn to_yes() {
        assert!((..5).contains(&3));
        assert!(3.is_within(&(..5)));
    }

    #[test]
    fn to_no() {
        assert!(!(..5).contains(&7));
        assert!(!7.is_within(&(..5)));
    }
}

