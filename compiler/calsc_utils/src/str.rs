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

/// Performs the Levenshtein distance algorithm:
/// https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein(a: &str, b: &str) -> usize {
    // Work on bytes for speed
    let a = a.as_bytes();
    let b = b.as_bytes();

    // Ensure b is the shorter string
    if a.len() < b.len() {
        return levenshtein_bytes(b, a);
    }

    levenshtein_bytes(a, b)
}

fn levenshtein_bytes(a: &[u8], b: &[u8]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0; b.len() + 1];

    for (i, &ac) in a.iter().enumerate() {
        curr[0] = i + 1;

        for (j, &bc) in b.iter().enumerate() {
            let cost = (ac != bc) as usize;

            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b.len()]
}
