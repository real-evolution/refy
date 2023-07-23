use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use serde::{Deserialize, Serialize};

/// A struct representing a tag.
#[derive(Clone, Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The unique identifier of the tag.
    pub id: Key<Self, uuid::Uuid>,

    /// The code of the tag.
    pub tag: String,

    /// The timestamp at which the tag was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
