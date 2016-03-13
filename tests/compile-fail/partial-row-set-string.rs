extern crate kudu;

fn main() {
    let mut row: kudu::PartialRow = unimplemented!();
    let value = String::new();
    row.set_copy_by_name("col", &value[..]);    // no error
    row.set_by_name("col", &value[..]);         //~ error: `value` does not live long enough
}
