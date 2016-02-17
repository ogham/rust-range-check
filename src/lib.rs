#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::borrow::Borrow;
use std::ops::{Range, RangeFrom, RangeTo};

mod bounds;
pub use bounds::{Bounds, Bounded};

mod result;
pub use result::{Check, Result};


pub trait Contains<T> {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool;
}

impl<T> Contains<T> for Range<T>
where T: PartialOrd {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool {
        (value.borrow() >= &self.start) && (value.borrow() < &self.end)
    }
}

impl<T> Contains<T> for RangeFrom<T>
where T: PartialOrd {
    fn contains<TRef: Borrow<T>>(&self, value: TRef) -> bool {
        value.borrow() >= &self.start
    }
}

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

