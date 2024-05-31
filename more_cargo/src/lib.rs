//! more_cargo
//! A example of useful cargo documentation and configurations
//!
//! The '!//' are a contained focument comment, which adds documentation to
//! The entire module.

/// This is a documentation comment.
/// *They* work with markdown and compile to _HTML_.
///
/// Adds one to the given number.
/// # Examples
/// ```
/// let arg = 5;
/// let answer = more_cargo::add_one(arg);
/// assert_eq!(6, answer);
/// ```
///
/// To generate the documentation, run cargo doc
/// to see the documentation, run cargo doc --open
///
/// # Tests
/// cargo test will also run the above doc code as a test
/// This is so we can see if the doc is in sync with the function.
pub fn add_one(x:i32) -> i32 {
    x + 1
}

pub use self::colors::PrimaryColor;
/// This is a submodule. normally, in order to access it,
/// a user needs to 'use more_cargo::colors::PrimaryColor'.
/// in this case, however, we have a pub 'use self::colors::PrimaryColor'.
/// Now users only need 'use more_cargo' to access Primary color.
pub mod colors {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}
