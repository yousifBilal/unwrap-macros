//! This tiny library provides unwrapping macros in situation where
//! the typical unwrapping methods for Result and Option in the standard
//! library comes short and the alternative is too verbose. Specifically when you want to have the `unwrap_or_else` logic
//! but need to `continue` or `return`.

//! # Usage
//!
//! This will log the error with eprintln! and skips the iteration.
//! ```
//! // with Result
//! let some_stuff = vec![
//!     Ok(1),
//!     Ok(2),
//!     Err(MyError::First),
//!     Ok(4),
//!     Err(MyError::Second),
//! ];
//! for thing in some_stuff {
//!     let val = unwrap_or_else!(Result, thing, continue);
//!     println!("{}", val);
//! }
//! ```
//! You can also supply a _closure-like_
//! argument:
//! ```
//! for thing in some_stuff {
//!     let val = unwrap_or_else!(Result, thing, |e| {
//!         // some code
//!         eprintln!("Custom message for error: {e}");
//!         continue,
//!     });
//!     println!("{}", val);
//! }
//! ```
//! ```
//! // with Option
//! let some_stuff = vec![
//!     Some(1),
//!     Some(2),
//!     None,
//!     Some(4),
//!     None,
//! ];
//! for thing in some_stuff {
//!     let val = unwrap_or_else!(Option, thing, continue);
//!     println!("{}", val);
//! }
//! ```
//! This will log "No value" when `None` is matched.
//! ```
//! for thing in some_stuff {
//!     let val = unwrap_or_else!(Option, thing, "No value", continue);
//!     println!("{}", val);
//! }
//! ```

#[macro_export]
macro_rules! unwrap_or_else {
    (Result, $x:expr, $y:expr) => {
        match $x {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                $y
            }
        }
    };

    ($x:expr, $y:expr) => {
        match $x {
            Ok(val) | Some(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                $y
            }
            None => $y,
        }
    };

    (Result, $x:expr, |$e:ident| $y:expr) => {
        match $x {
            Ok(val) => val,
            Err($e) => $y,
        }
    };

    (Option, $x:expr, $y:expr) => {
        match $x {
            Some(val) => val,
            None => $y,
        }
    };

    (Option, $x:expr, $y:expr, $z:expr) => {
        match $x {
            Some(val) => val,
            None => {
                eprintln!("{}", $y);
                $z
            }
        }
    };
}
