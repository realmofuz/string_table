#[inline]
/// Hashes the string into a u32.
/// This algorithm is the same as Java's `java.lang.String::hashCode()`, as it is speedy and will guarantee there are no duplicates.
pub fn string_hash(input: &str) -> u32 {
    let mut h: u32 = 0;
    for ch in input.chars() {
        h = h.wrapping_mul(31).wrapping_add(ch as u32);
    }
    h
}
