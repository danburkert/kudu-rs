pub trait BitSet {
    fn get_bit(&self, idx: usize) -> bool;
    fn set_bit(&mut self, idx: usize);
    fn clear_bit(&mut self, idx: usize);
    fn byte_len(num_bits: usize) -> usize {
        (num_bits + 7) / 8
    }
}

impl <T> BitSet for T where T: AsRef<[u8]> + AsMut<[u8]> {
    fn get_bit(&self, idx: usize) -> bool {
        self.as_ref()[idx >> 3] & (1 << (idx & 7)) > 0
    }

    fn set_bit(&mut self, idx: usize) {
        self.as_mut()[idx >> 3] |= 1 << (idx & 7)
    }

    fn clear_bit(&mut self, idx: usize) {
        self.as_mut()[idx >> 3] &= !(1 << (idx & 7))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let bitset = vec![0; BitSet::byte_len(16)];
        assert_eq!(bitset.get_bit(0), false);
    }
}
