use serde::{Deserialize, Serialize};

use crate::envelope::View;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ViewQuery {
    pub view: Option<View>,
}
