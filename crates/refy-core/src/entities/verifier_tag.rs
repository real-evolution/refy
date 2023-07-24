use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key};
use reddd_macros::Entity;
use serde::{Deserialize, Serialize};

use super::{Tag, Verifier};

/// A struct representing a verifier tags.
#[derive(Clone, Debug, Entity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierTag {
    /// The unique identifier of the verifier tags.
    pub id: Key<Self, uuid::Uuid>,

    /// The verifier id.
    pub verifier_id: <Verifier as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,

    /// The timestamp at which the ticket tags was created.
    pub created_at: DateTime<Utc>,
}
