use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use serde::{Deserialize, Serialize};

use super::{Tag, Ticket, Verifier};

/// A struct representing a verified ticket.
#[derive(Clone, Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedTickets {
    /// The unique identifier.
    pub id: Key<Self, uuid::Uuid>,

    /// The note of the struct.
    pub note: String,

    /// The verifier id.
    pub verifier_id: <Verifier as Entity>::Key,

    /// The ticket id.
    pub ticket_id: <Ticket as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,

    /// The timestamp at which the ticket was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
