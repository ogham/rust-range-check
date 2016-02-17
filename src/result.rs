use std::any::Any;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::result;

use bounds::{Bounds, Bounded};
use super::{Contains, Within};


/// Trait that provides early returns for failed range checks using `Result`s.
pub trait Check<R>: Sized {

    /// Checks whether `self` is within the given range. If it is, re-returns
    /// `self`. Otherwise, returns an `Error` that contains both the value and
    /// the range.
    ///
    /// Because it has to return at least part of them, this method consumes
    /// both `self` and the range.
    ///
    /// ### Examples
    ///
    /// ```
    /// use range_check::Check;
    ///
    /// assert!(24680.check_range(1..99999).is_ok());
    /// assert!(24680.check_range(1..9999).is_err());
    /// ```
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


/// The error that gets thrown when a `check_range` fails.
#[derive(PartialEq, Debug, Clone)]
pub struct Error<T> {

    /// The bounds of the range that was searched.
    pub allowed_range: Bounds<T>,

    /// The value that lies outside of the range.
    pub outside_value: T,
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


/// Type alias for a `Result` with an `Error` wrapping the result's error type.
pub type Result<T, E=T> = result::Result<T, Error<E>>;
