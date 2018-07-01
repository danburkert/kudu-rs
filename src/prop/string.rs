use std::{char, iter, mem};

use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Any {
    min_len: usize,
    max_len: usize,
}

impl Any {
    pub fn new(min_len: usize, max_len: usize) -> Any {
        Any { min_len, max_len }
    }
}

impl Strategy for Any {
    type Tree = BinarySearch;
    type Value = String;

    fn new_tree(
        &self,
        runner: &mut test_runner::TestRunner,
    ) -> Result<BinarySearch, test_runner::Reason> {
        Ok(BinarySearch::gen(self.min_len, self.max_len, runner.rng()))
    }
}

/// Shrinks a string towards "\0\0\0...", using binary search to find boundary points.
///
/// This is particularly useful when the string is treated as a lexicographic sequence of
/// characters by the application. Shrinking converges faster than the per-character shrinking of
/// the default proptest String value tree.
#[derive(Clone, Debug, PartialEq)]
pub struct BinarySearch {
    lo: Vec<u32>,
    curr: Vec<u32>,
    hi: Vec<u32>,
    shrink_idx: usize,
}

impl BinarySearch {
    // https://github.com/rust-lang/rust/blob/1.27.0/src/libcore/char/convert.rs#L207-L219
    const SURROGATE_PAIR_RANGE_OFFSET: u32 = 0xD800;
    const SURROGATE_PAIR_RANGE_LEN: u32 = 0xE000 - 0xD800;

    // increment and reposition do calculations on the lo, hi and curr strings as if they
    // were arbitrary-precision real numbers between [0, 1.0) expressed in this base.
    const RADIX: u32 = char::MAX as u32 - Self::SURROGATE_PAIR_RANGE_LEN + 1;

    pub fn new(start: &str) -> BinarySearch {
        let lo = Vec::new();
        let curr = Self::string_to_u32s(start);
        let hi = curr.clone();
        BinarySearch {
            lo,
            curr,
            hi,
            shrink_idx: 0,
        }
    }

    pub fn gen<R>(min_len: usize, max_len: usize, rng: &mut R) -> BinarySearch
    where
        R: Rng,
    {
        let lo = Vec::new();
        let curr = (0..rng.gen_range(min_len, max_len))
            .map(|_| rng.gen_range(0, Self::RADIX))
            .collect::<Vec<_>>();
        let hi = curr.clone();
        BinarySearch {
            lo,
            curr,
            hi,
            shrink_idx: 0,
        }
    }

    fn char_to_u32(c: char) -> u32 {
        let i = c as u32;
        i - (i >= Self::SURROGATE_PAIR_RANGE_OFFSET) as u32 * Self::SURROGATE_PAIR_RANGE_LEN
    }

    fn u32_to_char(i: u32) -> char {
        debug_assert!(i < Self::RADIX, "{:?} < {:?}", i, Self::RADIX);
        char::from_u32(
            i + (i >= Self::SURROGATE_PAIR_RANGE_OFFSET) as u32 * Self::SURROGATE_PAIR_RANGE_LEN,
        ).unwrap()
    }

    fn u32s_to_string(chars: &[u32]) -> String {
        chars.iter().cloned().map(Self::u32_to_char).collect()
    }

    fn string_to_u32s(s: &str) -> Vec<u32> {
        s.chars().map(Self::char_to_u32).collect()
    }

    fn increment(v: &mut [u32]) {
        for i in v.iter_mut().rev() {
            if *i < Self::RADIX - 1 {
                *i += 1;
                return;
            } else {
                *i = 0;
            }
        }
        unreachable!()
    }

    fn shrink(&mut self) -> bool {
        while self.shrink_idx < self.hi.len()
            && self.lo.get(self.shrink_idx).cloned().unwrap_or(0) == self.hi[self.shrink_idx]
        {
            let idx = self.shrink_idx;
            self.shrink_idx += 1;

            if self.lo.len() < idx || self.lo[idx..].iter().all(|&i| i == 0) {
                self.curr.clear();
                self.curr.extend_from_slice(&self.hi[..idx]);
                return true;
            }
        }
        false
    }

    fn unshrink(&mut self) -> bool {
        if self.shrink_idx > self.curr.len() {
            self.lo.resize(self.shrink_idx, 0);
            self.reposition();
            return true;
        }
        false
    }

    fn reposition(&mut self) {
        self.curr.clear();
        let mut carry = 0;
        for (&h, &l) in self.lo.iter().chain(iter::repeat(&0)).zip(self.hi.iter()) {
            let mut sum = h + l + carry * Self::RADIX;

            if sum >= Self::RADIX * 2 {
                // Overflow; increment the previous place.
                Self::increment(&mut self.curr);
                sum -= 2 * Self::RADIX;
            }

            carry = sum % 2;
            self.curr.push(sum / 2);
        }
    }
}

impl ValueTree for BinarySearch {
    type Value = String;

    fn current(&self) -> String {
        Self::u32s_to_string(&self.curr)
    }

    fn simplify(&mut self) -> bool {
        if self.lo >= self.hi {
            trace!("simplify (done): {:?}", self);
            return false;
        }

        mem::swap(&mut self.hi, &mut self.curr);

        if self.shrink() {
            trace!("simplify (shrunk): {:?}", self);
            return true;
        }

        self.reposition();
        trace!("simplified: {:?}", self);
        true
    }

    fn complicate(&mut self) -> bool {
        if self.curr >= self.hi {
            trace!("complicate (done): {:?}", self);
            return false;
        }

        if self.unshrink() {
            trace!("complicate (unshrunk): {:?}", self);
            return true;
        }

        mem::swap(&mut self.curr, &mut self.lo);
        Self::increment(&mut self.lo);

        if self.shrink() {
            trace!("complicate (shrunk): {:?}", self);
            return true;
        }

        self.reposition();
        trace!("complicated: {:?}", self);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    use proptest::prelude::*;
    use proptest::strategy::{check_strategy_sanity, CheckStrategySanityOptions, ValueTree};

    #[test]
    fn test_strategy() {
        let _ = env_logger::try_init();
        let mut options = CheckStrategySanityOptions::default();
        options.strict_complicate_after_simplify = false;
        check_strategy_sanity(Any::new(0, 16), Some(options));
    }

    fn converge_binary_search(value: &str, lower_bound: &str) -> Result<(), TestCaseError> {
        prop_assume!(value >= lower_bound);
        let mut search = BinarySearch::new(value);
        loop {
            let current = search.current();
            if current.as_str() >= lower_bound {
                if !search.simplify() {
                    break;
                }
            } else if !search.complicate() {
                break;
            }
        }

        let value = BinarySearch::string_to_u32s(value);
        let mut expected = BinarySearch::string_to_u32s(lower_bound);
        if value.len() < expected.len() {
            expected.truncate(value.len());
            BinarySearch::increment(&mut expected);
            while expected.last() == Some(&0) {
                expected.pop();
            }
        }
        prop_assert_eq!(
            &expected,
            &search.curr,
            "search did not converge: value: {:?}, lower_bound: {:?}, search: {:?}",
            value,
            expected,
            search
        );
        Ok(())
    }

    #[test]
    fn test_binary_search_converges() {
        let _ = env_logger::try_init();

        /*
        converge_binary_search("a\0\0\0", "0\u{63489}!").unwrap();
        converge_binary_search("i", "a").unwrap();

        converge_binary_search("z", "f").unwrap();
        converge_binary_search("zz", "ff").unwrap();
        converge_binary_search("zzz", "fff").unwrap();
        converge_binary_search("zzzz", "ffff").unwrap();

        converge_binary_search("z\0", "ff").unwrap();
        converge_binary_search("z\0\0", "fff").unwrap();
        converge_binary_search("z\0\0\0", "ffff").unwrap();

        converge_binary_search("zz", "f").unwrap();
        converge_binary_search("zzzz", "fff").unwrap();
        converge_binary_search("z\0\0\0\0\0\0\0", "ffff").unwrap();

        converge_binary_search("f", "").unwrap();
        converge_binary_search("ff", "").unwrap();
        converge_binary_search("fff", "").unwrap();
        */

        converge_binary_search("n¥7¢", "*\'/").unwrap();
    }

    proptest! {

        #[test]
        fn check_char_conversion(c in any::<char>()) {
            let _ = env_logger::try_init();
            let u32_ = BinarySearch::char_to_u32(c);
            prop_assert_eq!(c, BinarySearch::u32_to_char(u32_));
        }

        #[test]
        fn check_string_conversion(s in ".{0, 64}") {
            let _ = env_logger::try_init();
            let u32s = BinarySearch::string_to_u32s(&s);
            prop_assert_eq!(s, BinarySearch::u32s_to_string(&u32s));
        }

        #[test]
        fn check_binary_search_converges(value in ".{0, 8}", lower_bound in ".{0, 8}") {
            let _ = env_logger::try_init();
            converge_binary_search(&value, &lower_bound)?;
        }
    }
}
