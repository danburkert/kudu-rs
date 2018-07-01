use DataType;
use Row;
use Schema;

use super::binary;
use super::bool;
use super::string;

use proptest::{
    self, num,
    strategy::{self, Strategy, ValueTree},
};
use rand::Rng;

impl<'a> Row<'a> {
    pub fn arbitrary(schema: Schema) -> RowStrategy {
        RowStrategy { schema }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowStrategy {
    schema: Schema,
}

impl Strategy for RowStrategy {
    type Tree = RowTree;
    type Value = Row<'static>;
    fn new_tree(&self, runner: &mut proptest::test_runner::TestRunner) -> strategy::NewTree<Self> {
        debug!("RowStrategy::new_tree");
        let mut row = self.schema.new_row();
        for (idx, column) in self.schema.columns().iter().enumerate() {
            let rng = runner.rng();
            if column.is_nullable() && rng.gen::<bool>() {
                row.set_null(idx).unwrap();
            } else {
                unsafe {
                    match column.data_type() {
                        DataType::Bool => row.set_unchecked(idx, rng.gen::<bool>()),
                        DataType::Int8 => row.set_unchecked(idx, rng.gen::<i8>()),
                        DataType::Int16 => row.set_unchecked(idx, rng.gen::<i16>()),
                        DataType::Int32 => row.set_unchecked(idx, rng.gen::<i32>()),
                        DataType::Int64 | DataType::Timestamp => {
                            row.set_unchecked(idx, rng.gen::<i64>())
                        }
                        DataType::Float => row.set_unchecked(idx, f32::from_bits(rng.gen::<u32>())),
                        DataType::Double => {
                            row.set_unchecked(idx, f64::from_bits(rng.gen::<u64>()))
                        }
                        DataType::String => {
                            let string = (0..rng.gen_range::<usize>(0, 32))
                                .map(|_| rng.gen::<char>())
                                .collect::<String>();
                            row.set_unchecked(idx, string)
                        }
                        DataType::Binary => {
                            let bytes = (0..rng.gen_range::<usize>(0, 32))
                                .map(|_| rng.gen::<u8>())
                                .collect::<Vec<_>>();
                            row.set_unchecked(idx, bytes)
                        }
                    };
                }
            }
        }

        let column_tree_idx = 0;
        let column_tree = ColumnTree::new(&row, column_tree_idx);
        Ok(RowTree {
            row,
            column_tree_idx,
            column_tree,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RowTree {
    row: Row<'static>,
    column_tree_idx: usize,
    column_tree: ColumnTree,
}

impl proptest::strategy::ValueTree for RowTree {
    type Value = Row<'static>;

    fn current(&self) -> Row<'static> {
        self.row.clone()
    }

    fn simplify(&mut self) -> bool {
        debug!("RowTree::simplify");
        while !self.column_tree.simplify() {
            if self.column_tree_idx + 1 >= self.row.schema().primary_key().len() {
                return false;
            }

            self.column_tree_idx += 1;
            self.column_tree = ColumnTree::new(&self.row, self.column_tree_idx);
        }
        self.column_tree
            .set_current(&mut self.row, self.column_tree_idx);
        true
    }

    fn complicate(&mut self) -> bool {
        debug!("RowTree::complicate");
        if self.column_tree.complicate() {
            self.column_tree
                .set_current(&mut self.row, self.column_tree_idx);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
enum ColumnTree {
    Bool(bool::BoolValueTree),
    Int8(num::i8::BinarySearch),
    Int16(num::i16::BinarySearch),
    Int32(num::i32::BinarySearch),
    Int64(num::i64::BinarySearch),
    Float(num::f32::BinarySearch),
    Double(num::f64::BinarySearch),
    Binary(binary::BinarySearch),
    String(string::BinarySearch),
}

impl ColumnTree {
    fn new(row: &Row, idx: usize) -> ColumnTree {
        assert!(row.is_set(idx).unwrap());
        assert!(!row.is_null(idx).unwrap());
        match row.schema().columns()[idx].data_type() {
            DataType::Bool => ColumnTree::Bool(bool::BoolValueTree(row.get(idx).unwrap())),
            DataType::Int8 => ColumnTree::Int8(num::i8::BinarySearch::new(row.get(idx).unwrap())),
            DataType::Int16 => {
                ColumnTree::Int16(num::i16::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::Int32 => {
                ColumnTree::Int32(num::i32::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::Int64 | DataType::Timestamp => {
                ColumnTree::Int64(num::i64::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::Float => {
                ColumnTree::Float(num::f32::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::Double => {
                ColumnTree::Double(num::f64::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::Binary => {
                ColumnTree::Binary(binary::BinarySearch::new(row.get(idx).unwrap()))
            }
            DataType::String => {
                ColumnTree::String(string::BinarySearch::new(row.get(idx).unwrap()))
            }
        }
    }

    fn simplify(&mut self) -> bool {
        match self {
            ColumnTree::Bool(tree) => tree.simplify(),
            ColumnTree::Int8(tree) => tree.simplify(),
            ColumnTree::Int16(tree) => tree.simplify(),
            ColumnTree::Int32(tree) => tree.simplify(),
            ColumnTree::Int64(tree) => tree.simplify(),
            ColumnTree::Float(tree) => tree.simplify(),
            ColumnTree::Double(tree) => tree.simplify(),
            ColumnTree::Binary(tree) => tree.simplify(),
            ColumnTree::String(tree) => tree.simplify(),
        }
    }

    fn complicate(&mut self) -> bool {
        match self {
            ColumnTree::Bool(tree) => tree.complicate(),
            ColumnTree::Int8(tree) => tree.complicate(),
            ColumnTree::Int16(tree) => tree.complicate(),
            ColumnTree::Int32(tree) => tree.complicate(),
            ColumnTree::Int64(tree) => tree.complicate(),
            ColumnTree::Float(tree) => tree.complicate(),
            ColumnTree::Double(tree) => tree.complicate(),
            ColumnTree::Binary(tree) => tree.complicate(),
            ColumnTree::String(tree) => tree.complicate(),
        }
    }

    fn set_current(&self, row: &mut Row, idx: usize) {
        unsafe {
            match self {
                ColumnTree::Bool(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Int8(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Int16(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Int32(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Int64(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Float(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Double(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::Binary(tree) => row.set_unchecked(idx, tree.current()),
                ColumnTree::String(tree) => row.set_unchecked(idx, tree.current()),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    proptest! {
        #[test]
        fn check_debug_fmt(row in Schema::arbitrary().prop_flat_map(Row::arbitrary)) {
            let _ = env_logger::try_init();
            format!("{:?}", row);
        }

        #[test]
        fn check_eq(row in Schema::arbitrary().prop_flat_map(Row::arbitrary)) {
            let _ = env_logger::try_init();
            assert_eq!(row, row);
            assert_eq!(row.clone(), row);
            assert_eq!(row.clone(), row.into_owned());
        }
    }
}
