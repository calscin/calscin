//! String utilities

/// Prints a line of the given char starting at the given position
/// and ending at the given position.
///
/// # Example
/// ```
/// use calsc_utils::str::print_line;
///
/// let str = print_line(0, 5, '-');
/// let str2 = print_line(2, 5, '_');
///
/// assert_eq!(str, "-----".to_string());
/// assert_eq!(str2, "  ___".to_string());
/// ```
pub fn print_line(start: usize, end: usize, c: char) -> String {
    let mut str = String::new();

    for _ in 0..start {
        str.push(' ');
    }

    for _ in start..end {
        str.push(c);
    }

    str
}

/// Prints a blank line until the given end position
///
/// # Example
/// ```
/// use calsc_utils::str::print_blank_line;
///
/// let str = print_blank_line(5);
///
/// assert_eq!(str, "     ".to_string());
/// ```
///
pub fn print_blank_line(end: usize) -> String {
    let mut str = String::new();

    for _ in 0..end {
        str.push(' ');
    }

    str
}
