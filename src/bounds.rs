use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo};


/// The two bounds destructured from a Range value.
#[derive(PartialEq, Debug, Clone)]
pub struct Bounds<T> {

    /// The lower bound. `Range` and `RangeFrom` have one of these.
    pub lower: Option<T>,

    /// The upper bound. `Range` and `RangeTo` have one of these.
    pub upper: Option<T>,
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


/// Trait to get the bounds of ranges.
pub trait Bounded<T> {

    /// Returns a `Bounds` value containing the lower and upper bounds of this
    /// range, if present.
    fn bounds(self) -> Bounds<T>;
}


impl<T> Bounded<T> for Range<T> {
    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: Some(self.start),
            upper: Some(self.end),
        }
    }
}

impl<T> Bounded<T> for RangeFrom<T> {
    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: Some(self.start),
            upper: None,
        }
    }
}

impl<T> Bounded<T> for RangeTo<T> {
    fn bounds(self) -> Bounds<T> {
        Bounds {
            lower: None,
            upper: Some(self.end),
        }
    }
}
