use std::fmt;
use std::ops::Bound;


// We need this type to generalise over all the Range types.

/// The two bounds destructured from a Range value.
#[derive(PartialEq, Debug, Clone)]
pub struct Bounds<T> {

    /// The lower bound, created by `start_bound`.
    pub lower: Bound<T>,

    /// The upper bound, created by `end_bound`.
    pub upper: Bound<T>,
}

impl<T: fmt::Debug> fmt::Display for Bounds<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.lower {
            Bound::Included(n)  => write!(f, "{:?}", n)?,
            Bound::Excluded(n)  => write!(f, "{:?}=", n)?,  // surprisingly, not unreachable
            Bound::Unbounded    => {},
        }

        write!(f, "..")?;

        match &self.upper {
            Bound::Included(n)  => write!(f, "={:?}", n)?,
            Bound::Excluded(n)  => write!(f, "{:?}", n)?,
            Bound::Unbounded    => {},
        }

        Ok(())
    }
}

impl<T> Bounds<T>
{
    // This is basically an implementation of From in all but name.
    pub(crate) fn convert<U>(self) -> Bounds<U>
    where U: From<T>
    {
        let lower = match self.lower {
            Bound::Included(t)  => Bound::Included(U::from(t)),
            Bound::Excluded(t)  => Bound::Excluded(U::from(t)),
            Bound::Unbounded    => Bound::Unbounded,
        };

        let upper = match self.upper {
            Bound::Included(t)  => Bound::Included(U::from(t)),
            Bound::Excluded(t)  => Bound::Excluded(U::from(t)),
            Bound::Unbounded    => Bound::Unbounded,
        };

        Bounds { lower, upper }
    }
}

// http://github.com/rust-lang/rust/issues/61356
pub fn copy_bound<T: Copy>(bound: Bound<&T>) -> Bound<T> {
    match bound {
        Bound::Unbounded    => Bound::Unbounded,
        Bound::Included(n)  => Bound::Included(*n),
        Bound::Excluded(n)  => Bound::Excluded(*n),
    }
}
