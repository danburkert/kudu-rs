extern crate kudu;

fn main() {
    let row: kudu::PartialRow = unimplemented!();
    let value: &str = row.get_by_name("col").unwrap();
    drop(row); //~ error: cannot move out of `row` because it is borrowed
}
