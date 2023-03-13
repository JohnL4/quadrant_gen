use rand::prelude::*;

/// Throw 6-sided dice.
pub fn dice( num_dice: usize, dice_modifer: isize, rng: &mut ThreadRng) -> isize {
    let mut throw = 0;

    for _ in 1..=num_dice {
        throw += rng.gen_range(1..=6);
    }
    throw += dice_modifer;
    return throw;
}

/// Clamp n between min and max, inclusive.  If n < min, returns min; if n > max, returns max; else returns n (no-op).
/// Panics if min > max.
pub fn clamp( n: isize, min: isize, max: isize) -> isize {
    if (min > max) { panic!( "{}", format!( "min ({}) > max ({})", min, max)); }
    if (n < min) { return min; }
    if (n > max) { return max; }
    return n;
}

/// Converts single digit to uppercase hex code.  Will panic if digit is not in range 1..=16.
pub fn code( digit: isize) -> char {
    if digit < 0 || digit > 16 { panic!( "digit out of range 1..=16")}
    return char::from_digit(digit as u32, 16).unwrap().to_ascii_uppercase();
}