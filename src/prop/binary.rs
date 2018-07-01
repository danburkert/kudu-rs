use std::{iter, mem, u8};

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
    type Value = Vec<u8>;

    fn new_tree(
        &self,
        runner: &mut test_runner::TestRunner,
    ) -> Result<BinarySearch, test_runner::Reason> {
        Ok(BinarySearch::gen(self.min_len, self.max_len, runner.rng()))
    }
}

/// A `ValueTree` which shrinks a bytes value using binary search.
///
/// This is particularly useful when the bytes are treated as a lexicographic sequence by the
/// application. Shrinking converges much faster than the per-element shrinking done by the
/// default proptest vec strategy.
#[derive(Clone, Debug, PartialEq)]
pub struct BinarySearch {
    lo: Vec<u8>,
    curr: Vec<u8>,
    hi: Vec<u8>,
    shrink_idx: usize,
}

impl BinarySearch {
    const RADIX: u16 = u8::MAX as u16 + 1;

    pub fn new(start: &[u8]) -> BinarySearch {
        let lo = Vec::new();
        let curr = start.to_owned();
        let hi = start.to_owned();
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
            .map(|_| rng.gen())
            .collect::<Vec<_>>();
        let hi = curr.clone();
        BinarySearch {
            lo,
            curr,
            hi,
            shrink_idx: 0,
        }
    }

    fn increment(v: &mut Vec<u8>) {
        for i in v.iter_mut().rev() {
            if *i < u8::MAX {
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
            let h = u16::from(h);
            let l = u16::from(l);
            let mut sum = h + l + carry * Self::RADIX;

            if sum >= Self::RADIX * 2 {
                // Overflow; increment the previous place.
                Self::increment(&mut self.curr);
                sum -= 2 * Self::RADIX;
            }

            carry = sum % 2;
            self.curr.push((sum / 2) as u8);
        }
    }
}

impl ValueTree for BinarySearch {
    type Value = Vec<u8>;

    fn current(&self) -> Vec<u8> {
        self.curr.clone()
    }

    fn simplify(&mut self) -> bool {
        if self.lo >= self.hi {
            return false;
        }

        mem::swap(&mut self.hi, &mut self.curr);

        if self.shrink() {
            return true;
        }

        self.reposition();
        true
    }

    fn complicate(&mut self) -> bool {
        if self.curr >= self.hi {
            return false;
        }

        if self.unshrink() {
            return true;
        }

        mem::swap(&mut self.curr, &mut self.lo);
        Self::increment(&mut self.lo);

        if self.shrink() {
            return true;
        }

        self.reposition();
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

    fn converge_binary_search(value: &[u8], lower_bound: &[u8]) -> Result<(), TestCaseError> {
        prop_assume!(value >= lower_bound);
        let mut search = BinarySearch::new(value);
        loop {
            let current = search.current();
            if current.as_slice() >= lower_bound {
                if !search.simplify() {
                    break;
                }
            } else if !search.complicate() {
                break;
            }
        }

        let mut expected = lower_bound.to_owned();
        if value.len() < lower_bound.len() {
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
            lower_bound,
            search
        );
        Ok(())
    }

    #[test]
    fn test_binary_search_shrink() {
        let _ = env_logger::try_init();
        let mut search = BinarySearch {
            lo: vec![],
            curr: vec![0],
            hi: vec![0],
            shrink_idx: 0,
        };
        assert!(search.shrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![],
                curr: vec![],
                hi: vec![0],
                shrink_idx: 1
            }
        );
        assert!(!search.shrink());
        assert!(search.unshrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0],
                curr: vec![0],
                hi: vec![0],
                shrink_idx: 1
            }
        );
        assert!(!search.shrink());
        assert!(!search.unshrink());

        search = BinarySearch {
            lo: vec![0],
            curr: vec![0],
            hi: vec![0],
            shrink_idx: 0,
        };
        assert!(search.shrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0],
                curr: vec![],
                hi: vec![0],
                shrink_idx: 1
            }
        );
        assert!(!search.shrink());
        assert!(search.unshrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0],
                curr: vec![0],
                hi: vec![0],
                shrink_idx: 1
            }
        );
        assert!(!search.shrink());
        assert!(!search.unshrink());

        search = BinarySearch {
            lo: vec![0, 1, 2],
            curr: vec![0, 1, 2, 0, 0],
            hi: vec![0, 1, 2, 0, 0],
            shrink_idx: 0,
        };
        assert!(search.shrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0, 1, 2],
                curr: vec![0, 1, 2],
                hi: vec![0, 1, 2, 0, 0],
                shrink_idx: 4
            }
        );
        assert!(search.unshrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0, 1, 2, 0],
                curr: vec![0, 1, 2, 0, 0],
                hi: vec![0, 1, 2, 0, 0],
                shrink_idx: 4
            }
        );
        assert!(search.shrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0, 1, 2, 0],
                curr: vec![0, 1, 2, 0],
                hi: vec![0, 1, 2, 0, 0],
                shrink_idx: 5
            }
        );
        assert!(search.unshrink());
        assert_eq!(
            search,
            BinarySearch {
                lo: vec![0, 1, 2, 0, 0],
                curr: vec![0, 1, 2, 0, 0],
                hi: vec![0, 1, 2, 0, 0],
                shrink_idx: 5
            }
        );
        assert!(!search.shrink());
        assert!(!search.unshrink());
    }

    #[test]
    fn test_binary_search_converges() {
        let _ = env_logger::try_init();

        converge_binary_search(&[1, 0], &[0, 255, 0]).unwrap();
        converge_binary_search(&[3, 0], &[0, 0]).unwrap();

        converge_binary_search(&[122, 122], &[102]).unwrap();
        converge_binary_search(&[100], &[50]).unwrap();

        converge_binary_search(&[0], &[0]).unwrap();

        converge_binary_search(&[0], &[]).unwrap();

        converge_binary_search(b"z\0\0\0\0\0\0", b"ffff").unwrap();
        converge_binary_search(&[100], &[50]).unwrap();

        converge_binary_search(&[100], &[50]).unwrap();
        converge_binary_search(&[101], &[50]).unwrap();
        converge_binary_search(&[99], &[50]).unwrap();

        converge_binary_search(&[98], &[49]).unwrap();
        converge_binary_search(&[99], &[49]).unwrap();
        converge_binary_search(&[97], &[49]).unwrap();

        converge_binary_search(b"zz", b"ff").unwrap();
        converge_binary_search(b"zzz", b"fff").unwrap();
        converge_binary_search(b"zzzz", b"ffff").unwrap();

        converge_binary_search(b"z\0", b"ff").unwrap();
        converge_binary_search(b"z\0\0", b"fff").unwrap();
        converge_binary_search(b"z\0\0\0", b"ffff").unwrap();

        converge_binary_search(b"zz", b"f").unwrap();
        converge_binary_search(b"zzzz", b"fff").unwrap();
        converge_binary_search(b"z\0\0\0\0\0\0\0", b"ffff").unwrap();

        converge_binary_search(b"f", b"").unwrap();
        converge_binary_search(b"ff", b"").unwrap();
        converge_binary_search(b"fff", b"").unwrap();

        converge_binary_search(&[1], &[0, 0, 0, 0]).unwrap();

        converge_binary_search(&[0], &[]).unwrap();
        converge_binary_search(&[0, 0], &[]).unwrap();
        converge_binary_search(&[0, 0], &[0]).unwrap();
        converge_binary_search(&[0, 0, 0], &[0, 0]).unwrap();

        converge_binary_search(&[10, 20], &[7]).unwrap();
        converge_binary_search(&[10, 20, 30], &[7, 0, 5]).unwrap();
    }

    proptest! {
        #[test]
        fn check_binary_search_converges(value in any::<Vec<u8>>(), lower_bound in any::<Vec<u8>>()) {
            let _ = env_logger::try_init();
            converge_binary_search(&value, &lower_bound)?;
        }
    }
}
