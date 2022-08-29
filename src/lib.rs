//! The powerful function for real-number verification

mod automata;

use automata::FloatingPoint;

/// Verfiy if input string is a real number.
///
/// # Arguments
///
/// * `string` - A String reference that holds the name of the person
///
/// # Example
///
/// ```rust
/// use is_real::is_real;
/// let float = "-12.34e5".to_string();
/// assert_eq!(is_real(&float), true);
/// ```
pub fn is_real(s: &String) -> bool {
    FloatingPoint::accept(s)
}
