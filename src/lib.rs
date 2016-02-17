//! This is a little library that helps with range and bounds checking. It works
//! with Rust's standard `Range`, `RangeFrom`, and `RangeTo` types.
//!
//!
//! ## Checking whether a range contains a value
//!
//! The trait `Contains` is implemented on the range types. As long as the
//! data type in question is `PartialOrd`, it can be used to check whether a
//! value of that type is contained within a range:
//!
//! ```
//! use range_check::Contains;
//!
//! let range = 3000..5000;
//! assert!(range.contains(4123));
//!
//! let range = 10..;
//! assert!(range.contains(23));
//! ```
//!
//! There's also the `Within` trait, which does the same check, only with the
//! range as the argument:
//!
//! ```
//! use range_check::Within;
//!
//! assert!(4123.is_within(3000..5000));
//! assert!(23.is_within(10..));
//! ```
//!
//!
//! ## Failing early if a value is outside a range
//!
//! It can sometimes be more helpful to automatically return a failure case,
//! such as with the `try!` macro, than just check whether a value is inside a
//! range. The `Check` trait returns `Result`s that contain debugging
//! information for when a value doesn't lie within a range:
//!
//! ```
//! use range_check::Check;
//!
//! struct Clock {
//!     hour: i8,
//!     minute: i8,
//! }
//!
//! impl Clock {
//!     fn new(hour: i8, minute: i8) -> range_check::Result<Clock, i8> {
//!         Ok(Clock {
//!             hour:   try!(hour.check_range(0..24)),
//!             minute: try!(minute.check_range(0..60)),
//!         })
//!     }
//! }
//!
//! assert!(Clock::new(23, 59).is_ok());
//! assert!(Clock::new(24, 00).is_err());
//! ```
//!
//! Displaying the `Error` that gets returned in the error case shows you
//! exactly which range failed to be satisfied:
//!
//! ```
//! use std::string::ToString;
//! use range_check::Check;
//!
//! let failure = 13.check_range(0..10).unwrap_err();
//! assert_eq!(failure.to_string(), "value (13) outside of range (0 .. 10)");
//! ```


#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
//#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

use std::borrow::Borrow;
use std::ops::{Range, RangeFrom, RangeTo};

mod bounds;
pub use bounds::{Bounds, Bounded};

mod result;
pub use result::{Check, Result};


/// A trait for values that could contain another value, such as ranges.
pub trait Contains<T> {

    /// Whether this contains the given value.
    ///
    /// Supports both values and references as the thing to check.
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

