extern crate kudu;

use kudu::ColumnType;

fn main() {
    let mut row: kudu::PartialRow = unimplemented!();
    let value = String::new();
    kudu::ColumnType::set_copy_by_name(&value[..], &mut row, "col"); // no error
    kudu::ColumnType::set_by_name(&value[..], &mut row, "col");      //~ error: `value` does not live long enough
}
