use std::borrow::Cow;

use async_graphql::{ObjectType, SimpleObject, TypeName};

use crate::Record;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(name_type)]
pub struct Page<T: ObjectType + TypeName> {
    pub items: Vec<Record<T>>,
    pub page: i32,
    pub page_size: i32,
    pub total_items: i32,
    pub total_pages: i32,
}

impl<T: ObjectType + TypeName> TypeName for Page<T> {
    fn type_name() -> Cow<'static, str> {
        format!("{}Page", <T as TypeName>::type_name()).into()
    }
}
