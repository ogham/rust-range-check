#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::any::Any;
use std::borrow::Borrow;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo};
use std::result;

mod bounds;
pub use bounds::{Bounds, Bounded};


pub trait Contains<T> {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool;
}


// impls for Range...

impl<T> Contains<T> for Range<T>
where T: PartialOrd {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool {
        (value.borrow() >= &self.start) && (value.borrow() < &self.end)
    }
}

// impls for RangeFrom...

impl<T> Contains<T> for RangeFrom<T>
where T: PartialOrd {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool {
        value.borrow() >= &self.start
    }
}


// impls for RangeTo...

impl<T> Contains<T> for RangeTo<T>
where T: PartialOrd {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool {
        value.borrow() < &self.end
    }
}



pub trait Within<R, RRef: Borrow<R>>: Sized {
    fn is_within(&self, range: RRef) -> bool;
}

impl<T, R> Within<R, R> for T
where R: Contains<T> {
    fn is_within(&self, range: R) -> bool {
        range.borrow().contains(self)
    }
}

impl<'a, T, R> Within<R, &'a R> for T
where R: Contains<T> {
    fn is_within(&self, range: &'a R) -> bool {
        range.borrow().contains(self)
    }
}


pub trait Check<R>: Sized {
    fn check_range(self, range: R) -> Result<Self>;
}

impl<T, R> Check<R> for T
where R: Contains<T> + Bounded<T> {
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
        assert!((1..5).contains(3));
        assert!(3.is_within(1..5));
    }

    #[test]
    fn no() {
        assert!(!(1..5).contains(&7));
        assert!(!(7.is_within(1..5)));
    }

    #[test]
    fn from_yes() {
        assert!((1..).contains(&3));
        assert!(3.is_within(1..));
    }

    #[test]
    fn from_no() {
        assert!(!(1..).contains(&-7));
        assert!(!(-7).is_within(1..));
    }

    #[test]
    fn to_yes() {
        assert!((..5).contains(&3));
        assert!(3.is_within(..5));
    }

    #[test]
    fn to_no() {
        assert!(!(..5).contains(&7));
        assert!(!7.is_within(&(..5)));
    }
}

