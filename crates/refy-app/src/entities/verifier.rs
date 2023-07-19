use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use serde::{Deserialize, Serialize};

/// A struct representing a verifier.
#[derive(Clone, Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verifier {
    /// The unique identifier of the verifier.
    pub id: Key<Self, uuid::Uuid>,

    /// The display name of the verifier.
    pub display_name: String,

    /// The unique identifier of the owner of the verifier.
    pub principle_id: uuid::Uuid,

    /// The timestamp at which the verifier was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
