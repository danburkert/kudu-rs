extern crate kudu;

fn main() {
    let mut creator: kudu::TableCreator = unimplemented!();
    let schema: kudu::Schema = unimplemented!();
    creator.schema(&schema); //~ error: `schema` does not live long enough
}
