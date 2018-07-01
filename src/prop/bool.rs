use std::mem;

use proptest::strategy::ValueTree;

/// A `ValueTree` to shrink booleans to false.
#[derive(Clone, Copy, Debug)]
pub struct BoolValueTree(pub bool);

impl ValueTree for BoolValueTree {
    type Value = bool;
    fn current(&self) -> bool {
        self.0
    }
    fn simplify(&mut self) -> bool {
        mem::replace(&mut self.0, false)
    }
    fn complicate(&mut self) -> bool {
        !mem::replace(&mut self.0, true)
    }
}
