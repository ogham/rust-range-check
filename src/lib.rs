#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::ops::Range;
use std::result;


pub trait RangeExt: Sized {

    /// Returns whether this value exists within the given range of values.
    fn is_within(&self, range: &Range<Self>) -> bool;

    fn check_range(self, range: Range<Self>) -> Result<Self> {
        if self.is_within(&range) {
            Ok(self)
        }
        else {
            Err(Error {
                allowed_range: range,
                outside_value: self,
            })
        }
    }
}

// Define RangeExt on *anything* that can be compared, though it’s only
// really ever used for numeric ranges...

impl<T> RangeExt for T where T: PartialOrd<T> {
    fn is_within(&self, range: &Range<Self>) -> bool {
        *self >= range.start && *self < range.end
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct Error<T> {
    allowed_range: Range<T>,
    outside_value: T,
}

pub type Result<T> = result::Result<T, Error<T>>;
