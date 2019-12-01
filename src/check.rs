use std::error::Error as ErrorTrait;
use std::ops::RangeBounds;
use std::fmt;

use bounds::{Bounds, copy_bound};


/// Trait that provides early returns for failed range checks using the
/// `Result` type.
pub trait Check<R: RangeBounds<Self>>: Sized + PartialOrd + Copy {

    /// Checks whether `self` is within the given range. If it is, re-returns
    /// `self`. Otherwise, returns an `Error` that contains both the value and
    /// the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use range_check::Check;
    ///
    /// assert!(24680.check_range(1..99999).is_ok());
    /// assert!(24680.check_range(1..9999).is_err());
    /// ```
    fn check_range(self, range: R) -> Result<Self, OutOfRangeError<Self>>;
}

impl<T, R> Check<R> for T
where R: RangeBounds<T>,
      T: PartialOrd + Copy,
{
    fn check_range(self, range: R) -> Result<Self, OutOfRangeError<Self>> {
        if range.contains(&self) {
            Ok(self)
        }
        else {
            let bounds = Bounds {
                lower: copy_bound(range.start_bound()),
                upper: copy_bound(range.end_bound()),
            };

            Err(OutOfRangeError { allowed_range: bounds, outside_value: self })
        }
    }
}


/// The error that gets thrown when a `check_range` fails.
#[derive(PartialEq, Debug, Clone)]
pub struct OutOfRangeError<T> {

    /// The bounds of the range that was searched.
    pub allowed_range: Bounds<T>,

    /// The value that lies outside of the range.
    pub outside_value: T,
}

impl<T: fmt::Debug> ErrorTrait for OutOfRangeError<T> {
    fn description(&self) -> &str {
        "value outside of range"
    }
}

impl<T: fmt::Debug> fmt::Display for OutOfRangeError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value ({:?}) outside of range ({})",
            self.outside_value, self.allowed_range)
    }
}

impl<T> OutOfRangeError<T> {

    /// Converts this error to an error with the same values as another type.
    /// The other type must be `From`-convertible from this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use range_check::{Check, OutOfRangeError};
    ///
    /// let err: OutOfRangeError<i16> = 24680.check_range(1..9999).unwrap_err();
    /// let err: OutOfRangeError<i32> = err.generify();
    /// ```
    pub fn generify<U: From<T>>(self) -> OutOfRangeError<U> {
        OutOfRangeError {
            allowed_range: self.allowed_range.convert(),
            outside_value: self.outside_value.into(),
        }
    }
}
