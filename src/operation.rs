use Result;
use Row;
use TableId;
use key;
use meta_cache::MetaCache;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationKind {
    Insert,
    Update,
    Upsert,
    Delete,
}

pub struct Operation {
    meta_cache: MetaCache,
    row: Row,
    kind: OperationKind,
}

impl Operation {

    pub fn kind(&self) -> OperationKind {
        self.kind
    }

    pub fn table(&self) -> TableId {
        self.meta_cache.table()
    }

    pub fn row(&self) -> &Row {
        &self.row
    }

    pub fn mut_row(&mut self) -> &mut Row {
        &mut self.row
    }

    pub fn partition_key(&self) -> Result<Vec<u8>> {
        let mut key = Vec::new();
        try!(key::encode_partition_key(&self.meta_cache.partition_schema(), &self.row, &mut key));
        Ok(key)
    }

    pub fn meta_cache(&self) -> &MetaCache {
        &self.meta_cache
    }
}
