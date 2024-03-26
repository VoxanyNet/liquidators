use diff::Diff;
use serde::{Deserialize, Serialize};

pub type Bytes = [u8; 16];

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Diff)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Uuid(Bytes);

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self {
            0: value.into_bytes()
        }
    }
}

impl From<&uuid::Uuid> for Uuid {
    fn from(value: &uuid::Uuid) -> Self {
        Self {
            0: value.into_bytes()
        }
    }
}

impl From<&mut uuid::Uuid> for Uuid {
    fn from(value: &mut uuid::Uuid) -> Self {
        Self {
            0: value.into_bytes()
        }
    }
}

impl Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.0)
    }
}
