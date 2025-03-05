use sqlx::{
    types::chrono::{DateTime, Utc},
    ColumnIndex, Decode, Row, Type,
};

use crate::{
    collection::Metadata as CollectionMetadata, format::Serializable, queue::Metadata as QueueMetadata, record::Record,
    record::ToRecord, types::State, Entity, Result,
};

pub trait RecordExt {
    fn select_fields() -> &'static str;
}

impl<T: Entity> RecordExt for Record<T, CollectionMetadata> {
    fn select_fields() -> &'static str {
        "id, data, state, keywords"
    }
}


impl<T, R> ToRecord<T, CollectionMetadata> for R
where
    R: Row + Send + Sync + Unpin + 'static,
    T: Entity,
    usize: ColumnIndex<R>,
    T::Id: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    DateTime<Utc>: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    State: for<'a> Decode<'a, R::Database> + Type<R::Database>,
    for<'a> &'a str: Decode<'a, R::Database> + Type<R::Database>,
    Vec<String>: for<'a> Decode<'a, R::Database> + Type<R::Database>,
{
    fn to_record(&self) -> Result<Record<T, CollectionMetadata>> {
        Ok(Record {
            id: self.try_get(0)?,
            data: <T as Entity>::Format::deserialize(self.try_get(1)?)?,
            metadata: CollectionMetadata {
                state: self.try_get(2)?,
                keywords: self.try_get(3)?,
            },
        })
    }
}
