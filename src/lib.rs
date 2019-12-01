//! This is a little library that helps with range and bounds checking. It works
//! with Rust’s standard `Range` types.
//!
//!
//! Range checking in the stdlib
//! ----------------------------
//!
//! Rust’s standard library allows you to test whether a range contains
//! a specified value:
//!
//! ```
//! // Range checking with std::ops
//! assert_eq!((0..24).contains(&23), true);
//! assert_eq!((0..24).contains(&24), false);
//! ```
//!
//! For more information, see the
//! [official Rust documentation for `std::ops::RangeBounds`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html).
//!
//!
//! Range checking with this crate
//! ------------------------------
//!
//! The `range_check` crate provides the [`Check`](trait.Check.html) trait that has a function
//! `check_range`, which returns a [`Result`](type.Result.html) instead of a `bool`.
//!
//! If the value exists within the range, it will return the value as an
//! `Ok` variant:
//!
//! ```
//! use range_check::Check;
//!
//! assert_eq!(24680.check_range(1..99999),
//!            Ok(24680));
//! ```
//!
//! If the value does _not_ exist within the range, it will be returned
//! inside an [`OutOfRangeError`](struct.OutOfRangeError.html) error variant:
//!
//! ```
//! use range_check::Check;
//!
//! assert_eq!(24680.check_range(1..9999).unwrap_err().to_string(),
//!            "value (24680) outside of range (1..9999)");
//! ```
//!
//! Failing early if a value is outside a range
//! -------------------------------------------
//!
//! When testing multiple values, it can sometimes be helpful to
//! automatically return when one of them is outside a range.
//!
//! In this example, we use the `?` operator to return early:
//!
//! ```
//! use range_check::{Check, OutOfRangeError};
//!
//! struct Clock {
//!     hour: i8,
//!     minute: i8,
//! }
//!
//! impl Clock {
//!     fn new(hour: i8, minute: i8) -> Result<Clock, OutOfRangeError<i8>> {
//!         Ok(Clock {
//!             hour: hour.check_range(0..24)?,
//!             minute: minute.check_range(0..60)?,
//!         })
//!     }
//! }
//!
//! assert!(Clock::new(23, 59).is_ok());
//! assert!(Clock::new(23, 60).is_err());
//! assert!(Clock::new(24, 00).is_err());
//! ```
//!
//! It becomes a problem when the values being tested are of different types,
//! as there can only be one type as the error `Result` from the function.
//!
//! As long as the types can be converted using the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
//! trait, you can convert the error using the
//! [`OutOfRangeError::generify`](struct.OutOfRangeError.html#method.generify)
//! function. In the first call in this example, we convert the error from
//! containing an `i8` to an `i16`:
//!
//! ```
//! use range_check::{Check, OutOfRangeError};
//!
//! struct Clock {
//!     second: i8,
//!     millisecond: i16,
//! }
//!
//! impl Clock {
//!     fn new(second: i8, millisecond: i16) -> Result<Clock, OutOfRangeError<i16>> {
//!         Ok(Clock {
//!             second: second.check_range(0..60).map_err(OutOfRangeError::generify)?,
//!             millisecond: millisecond.check_range(0..1000)?,
//!         })
//!     }
//! }
//!
//! assert!(Clock::new(45, 576).is_ok());
//! assert!(Clock::new(49, 23456).is_err());
//! assert!(Clock::new(61, 0).is_err());
//! ```


#![crate_name = "range_check"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

mod check;
pub use check::{Check, OutOfRangeError};

mod bounds;
