/// Convert a Unicode character to ASCII decimal number
/// Returns None if the character is outside ASCII range (0-127)
pub fn unicode_to_ascii_decimal(ch: char) -> Option<u8> {
    let code_point = ch as u32;
    if code_point <= 127 {
        Some(code_point as u8)
    } else {
        None
    }
}

/// Convert a Unicode code point to ASCII decimal number
/// Returns None if the code point is outside ASCII range (0-127)
pub fn unicode_codepoint_to_ascii_decimal(code_point: u32) -> Option<u8> {
    if code_point <= 127 {
        Some(code_point as u8)
    } else {
        None
    }
}

/// Convert a string character to ASCII decimal, with fallback options
pub fn char_to_ascii_with_fallback(ch: char, fallback: u8) -> u8 {
    unicode_to_ascii_decimal(ch).unwrap_or(fallback)
}
