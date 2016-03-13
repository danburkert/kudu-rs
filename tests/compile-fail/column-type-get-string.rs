extern crate kudu;

fn main() {
    let mut row: kudu::PartialRow = unimplemented!();
    let value: &str = kudu::ColumnType::get_by_name(&mut row, "col").unwrap();
    drop(row); //~ error: cannot move out of `row` because it is borrowed
}
