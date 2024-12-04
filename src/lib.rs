pub mod grid;

/// Parses the next `DIGITS` in the string as a u32
pub fn parse_u32<const DIGITS: usize>(input: &str, pos: &mut usize) -> u32 {
    let input = &input.as_bytes()[*pos..*pos + DIGITS];
    let mut num = 0;
    for i in (0..DIGITS).rev() {
        let digit = (input[i] & 0xf) as u32;
        let mult = 10_u32.pow((DIGITS - i - 1) as u32);
        num += digit * mult;
    }
    *pos += DIGITS;
    num
}
