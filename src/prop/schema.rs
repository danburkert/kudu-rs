use Column;
use CompressionType;
use DataType;
use EncodingType;
use Schema;
use SchemaBuilder;

use proptest;
use proptest::prelude::*;

impl Column {
    pub fn arbitrary_with<D, N>(
        data_type: D,
        /*TODO*/ _is_nullable: N,
    ) -> impl Strategy<Value = Column>
    where
        D: Strategy<Value = DataType>,
        N: Strategy<Value = bool>,
    {
        // To reduce the search space, column attributes are treated as independent when
        // possible, and the name is simplified last (since the search space of strings is much
        // greater than for the other attributes).
        data_type
            .prop_flat_map(|data_type| {
                (
                    Just(data_type),
                    EncodingType::arbitrary(data_type).no_shrink(),
                )
            }).prop_ind_flat_map2(|_| CompressionType::arbitrary().no_shrink())
            .prop_ind_flat_map2(|_| "[a-z0-9_]{1, 16}".no_shrink())
            .prop_map(|(((data_type, encoding), compression), name)| {
                Column::new(name, data_type)
                    .set_is_nullable(false)
                    .set_compression(compression)
                    .set_encoding(encoding)
            })
    }
    pub fn arbitrary() -> impl Strategy<Value = Column> {
        Column::arbitrary_with(DataType::arbitrary(), proptest::bool::ANY)
    }
    pub fn arbitrary_primary_key() -> impl Strategy<Value = Column> {
        Column::arbitrary_with(DataType::arbitrary_primary_key(), Just(false))
    }
}

impl Schema {
    pub fn arbitrary() -> impl Strategy<Value = Schema> {
        proptest::collection::vec(Column::arbitrary_primary_key(), 1..=16)
            .prop_ind_flat_map2(|pk_columns| {
                proptest::collection::vec(Column::arbitrary(), 0..(200 - pk_columns.len()))
            }).prop_filter("duplicate column name", |(pk_columns, columns)| {
                let mut column_names = pk_columns
                    .iter()
                    .chain(columns.iter())
                    .map(Column::name)
                    .collect::<Vec<_>>();
                column_names.sort_unstable();
                column_names.dedup();
                column_names.len() == pk_columns.len() + columns.len()
            }).prop_map(|(pk_columns, columns)| {
                info!("Schema::arbitrary new schema");
                let mut schema = SchemaBuilder::new();
                schema = schema.set_primary_key(
                    pk_columns
                        .iter()
                        .map(Column::name)
                        .map(ToString::to_string)
                        .collect::<Vec<_>>(),
                );

                for column in pk_columns.into_iter().chain(columns.into_iter()) {
                    schema = schema.add_column(column);
                }

                schema.build().unwrap()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    proptest! {
        #[test]
        fn check_schema_eq(schema in Schema::arbitrary()) {
            let _ = env_logger::try_init();
            assert_eq!(schema, schema);
        }
    }
}
