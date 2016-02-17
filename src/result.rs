use std::any::Any;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::result;

use bounds::{Bounds, Bounded};
use super::{Contains, Within};


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


pub type Result<T, E=T> = result::Result<T, Error<E>>;
