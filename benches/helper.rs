#[cfg(not(tarpaulin_include))]
#[inline]
pub fn next_ascii_char(c: char, offset: u8) -> Option<char> {
    if c.is_ascii() {
        let ascii_val = c as u8;
        if ascii_val + offset <= u8::MAX {
            Some((ascii_val + offset) as char)
        } else {
            None // Max ASCII value reached
        }
    } else {
        None // Not an ASCII character
    }
}
