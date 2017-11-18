/// Returns the number of bytes required to hold a number of bits.
pub fn len(num_bits: usize) -> usize {
    (num_bits + 7) / 8
}

/// Returns the value of the bit at the index.
pub fn get(bits: &[u8], idx: usize) -> bool {
    bits[idx >> 3] & (1 << (idx & 7)) > 0
}

/// Sets the value of the bit at the index to 1.
pub fn set(bits: &mut [u8], idx: usize) {
    bits[idx >> 3] |= 1 << (idx & 7)
}

/// Sets the value of the bit at the index to 0.
pub fn clear(bits: &mut [u8], idx: usize) {
    bits[idx >> 3] &= !(1 << (idx & 7))
}
