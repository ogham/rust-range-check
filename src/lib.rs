#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::ops::Range;
use std::result;


pub trait RangeExt: Sized {

    /// Returns whether this value exists within the given range of values.
    fn is_within(&self, range: Range<Self>) -> bool;

    fn check_range(self, range: Range<Self>) -> Result<Self> {
        if self.is_within(range) {
            Ok(self)
        }
        else {
            Err(Error)
        }
    }
}

// Define RangeExt on *anything* that can be compared, though it’s only
// really ever used for numeric ranges...

impl<T> RangeExt for T where T: PartialOrd<T> {
    fn is_within(&self, range: Range<Self>) -> bool {
        *self >= range.start && *self < range.end
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Error;

pub type Result<T> = result::Result<T, Error>;