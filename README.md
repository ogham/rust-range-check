# rust-range-check [![range-check on crates.io](http://meritbadge.herokuapp.com/range-check)](https://crates.io/crates/range_check) [![Build status](https://travis-ci.org/ogham/rust-range-check.svg?branch=master)](https://travis-ci.org/ogham/rust-range-check) [![Coverage status](https://coveralls.io/repos/ogham/rust-range-check/badge.svg?branch=master&service=github)](https://coveralls.io/github/ogham/rust-range-check?branch=master)

This is a little library that helps with range and bounds checking. It works with Rust’s standard `Range`, `RangeFrom`, and `RangeTo` types.

### [View the Rustdoc](http://ogham.rustdocs.org/range_check)


# Installation

This crate works with [Cargo](http://crates.io). Add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
range_check = "0.1"
```


## Checking whether a range contains a value

The trait `Contains` is implemented on the range types. As long as the data type in question is `PartialOrd`, it can be used to check whether a value of that type is contained within a range:

```rust
use range_check::Contains;

let range = 3000..5000;
assert!(range.contains(4123));

let range = 10..;
assert!(range.contains(23));
```

There’s also the `Within` trait, which does the same check, only with the range as the argument:

```rust
use range_check::Within;

assert!(4123.is_within(3000..5000));
assert!(23.is_within(10..));
```


## Failing early if a value is outside a range

It can sometimes be more helpful to automatically return a failure case, such as with the `try!` macro, than just check whether a value is inside a range. The `Check` trait returns `Result`s that contain debugging information for when a value doesn’t lie within a range:

```rust
use range_check::Check;

struct Clock {
    hour: i8,
    minute: i8,
}

impl Clock {
    fn new(hour: i8, minute: i8) -> range_check::Result<Clock, i8> {
        Ok(Clock {
            hour:   try!(hour.check_range(0..24)),
            minute: try!(minute.check_range(0..60)),
        })
    }
}

assert!(Clock::new(23, 59).is_ok());
assert!(Clock::new(24, 00).is_err());
```

Displaying the `Error` that gets returned in the error case shows you exactly which range failed to be satisfied:

```rust
use std::string::ToString;
use range_check::Check;

let failure = 13.check_range(0..10).unwrap_err();
assert_eq!(failure.to_string(), "value (13) outside of range (0 .. 10)");
```