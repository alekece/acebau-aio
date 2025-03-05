use derive_more::{Deref, DerefMut};

use crate::{Entity, Result};

pub trait ToRecord<T: Entity, M> {
    fn to_record(&self) -> Result<Record<T, M>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Record<T: Entity, M> {
    pub(crate) id: T::Id,
    #[deref]
    #[deref_mut]
    pub(crate) data: T,
    pub(crate) metadata: M,
}

impl<T: Entity, M> Record<T, M> {
    fn id(&self) -> &<T as Entity>::Id {
        &self.id
    }

    fn into_inner(self) -> T {
        self.data
    }
}
