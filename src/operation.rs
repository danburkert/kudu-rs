use std::fmt;

use Error;
use Row;
use TableId;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationKind {
    Insert,
    Update,
    Upsert,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operation {
    table: TableId,
    row: Row,
    kind: OperationKind,
}

impl Operation {
    pub fn kind(&self) -> OperationKind {
        self.kind
    }

    pub fn table(&self) -> TableId {
        self.table
    }

    pub fn row(&self) -> &Row {
        &self.row
    }

    pub fn mut_row(&mut self) -> &mut Row {
        &mut self.row
    }
}

pub type OperationResult = Result<(), OperationError>;

#[derive(Debug)]
pub struct OperationError {
    pub row: Row,
    pub error: Error,
}

impl ::std::error::Error for OperationError {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        self.error.cause()
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
