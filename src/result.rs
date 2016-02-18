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
            Err(Error::new(self, range))
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


impl<T> Error<T> {

    /// Creates a new `Error` using the given value and the bounds of the
    /// given range.
    ///
    /// This gets used by `check_range`, but may need to be called yourself if
    /// you're implementing, say, your own number-to-enum-variant constructor:
    ///
    /// ### Examples
    ///
    /// ```
    /// use range_check::{Error, Result};
    ///
    /// #[derive(Debug)]
    /// enum Number { One, Two, Three }
    ///
    /// impl Number {
    ///     fn from_u8(num: u8) -> Result<Number, u8> {
    ///         Ok(match num {
    ///             1 => Number::One,
    ///             2 => Number::Two,
    ///             3 => Number::Three,
    ///             n => return Err(Error::new(n, 1..4)),
    ///         })
    ///     }
    /// }
    ///
    /// let error = Number::from_u8(4).unwrap_err();
    /// assert_eq!(error.outside_value, 4);
    /// ```
    pub fn new<R: Bounded<T>>(value: T, range: R) -> Error<T> {
        Error {
            outside_value: value,
            allowed_range: range.bounds(),
        }
    }

    /// Converts every value present in the error using `Into::into`. This
    /// usually has the effect of making the error more generic: you can use
    /// the *same* type to work with ranges of *different* types.
    ///
    /// For example, you could need to validate one `i8` and one `i32`.
    /// Ordinarily you wouldn’t be able to check both types in the same
    /// function, as one would return an `Error<i8>`, and the other an
    /// `Error<i32>`. But because there exists `impl From<i8> for i32`,
    /// we can use that to convert the numbers to a different type.
    ///
    /// ### Examples
    ///
    /// The function below checks both an `i8` and an `i32`, but returns a
    /// `range_check::Error<i32>` because of the `From` implementation.
    ///
    /// ```
    /// use range_check::{self, Check};
    ///
    /// enum Error {
    ///     OutOfRange(range_check::Error<i64>),
    /// }
    ///
    /// impl<E> From<range_check::Error<E>> for Error
    /// where i64: From<E> {
    ///     fn from(original: range_check::Error<E>) -> Error {
    ///         Error::OutOfRange(original.generify())
    ///     }
    /// }
    ///
    /// fn tiny_clock(hour: i8, minute: i32) -> Result<(i8, i32), Error> {
    ///     let hour = try!(hour.check_range(0..24));
    ///     let minute = try!(minute.check_range(0..60));
    ///     Ok((hour, minute))
    /// }
    ///
    /// assert!(tiny_clock(23, 59).is_ok());
    /// assert!(tiny_clock(24, 00).is_err());
    /// ```
    pub fn generify<U>(self) -> Error<U>
    where U: From<T> {
        Error {
            outside_value: U::from(self.outside_value),
            allowed_range: Bounds {
                lower: self.allowed_range.lower.map(From::from),
                upper: self.allowed_range.upper.map(From::from),
            },
        }
    }
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


/// Type alias for a `Result` with an `Error` wrapping the result’s error type.
pub type Result<T, E=T> = result::Result<T, Error<E>>;
