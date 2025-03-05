use derive_more::{Deref, DerefMut};
use sqlx::{error::BoxDynError, Database, Decode, Type};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Uri(Url);

impl<DB> Type<DB> for Uri
where
    DB: Database,
    String: Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as Type<DB>>::type_info()
    }
}

impl<'a, DB> Decode<'a, DB> for Uri
where
    DB: Database,
    &'a str: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;

        Ok(Url::parse(value).map_err(Box::new).map(Uri)?)
    }
}
