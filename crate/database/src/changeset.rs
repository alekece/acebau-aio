use crate::Status;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Changeset<T> {
    status: Option<Status>,
    data: Option<T>,
}

impl<T> Changeset<T> {
    pub fn is_empty(&self) -> bool {
        self.status.is_none() && self.data.is_none()
    }

    pub fn status(&self) -> Option<Status> {
        self.status
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}
