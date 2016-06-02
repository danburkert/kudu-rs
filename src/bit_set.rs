#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    data: Box<[u8]>,
}

impl BitSet {
    pub fn with_capacity(num_bits: usize) -> BitSet {
        BitSet {
            data: vec![0; (num_bits + 7) / 8].into_boxed_slice()
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        self.data[idx >> 3] & (1 << (idx & 7)) > 0
    }

    pub fn insert(&mut self, idx: usize) {
        self.data[idx >> 3] |= 1 << (idx & 7)
    }

    pub fn remove(&mut self, idx: usize) {
        self.data[idx >> 3] &= !(1 << (idx & 7))
    }

    pub fn data(&self) -> &[u8] {
        &*self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bitset = BitSet::with_capacity(16);
        assert_eq!(bitset.get(0), false);
    }
}
