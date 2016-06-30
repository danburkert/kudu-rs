//! Utility functions for working with keys.

use byteorder::{ByteOrder, NativeEndian};

/// Murmur2 hash implementation returning 64-bit hashes.
pub fn murmur2_64(mut data: &[u8], seed: u64) -> u64 {
    let m: u64 = 0xc6a4a7935bd1e995;
    let r: u8 = 47;

    let mut h : u64 = seed ^ ((data.len() as u64).wrapping_mul(m));

    while data.len() >= 8 {
        let mut k = NativeEndian::read_u64(data);

        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h ^= k;
        h = h.wrapping_mul(m);

        data = &data[8..];
    };

    let len = data.len();
    if len > 6 { h ^= (data[6] as u64) << 48; }
    if len > 5 { h ^= (data[5] as u64) << 40; }
    if len > 4 { h ^= (data[4] as u64) << 32; }
    if len > 3 { h ^= (data[3] as u64) << 24; }
    if len > 2 { h ^= (data[2] as u64) << 16; }
    if len > 1 { h ^= (data[1] as u64) << 8; }
    if len > 0 { h ^= data[0] as u64;
                 h = h.wrapping_mul(m) }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;
    h
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_murmur2_64() {
        assert_eq!(7115271465109541368, murmur2_64(b"ab", 0));
        assert_eq!(2601573339036254301, murmur2_64(b"abcdefg", 0));
        assert_eq!(3575930248840144026, murmur2_64(b"quick brown fox", 42));
    }
}
