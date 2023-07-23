use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key};
use reddd_macros::Entity;
use serde::{Deserialize, Serialize};

use super::{Tag, Ticket};

/// A struct representing a ticket tags.
#[derive(Clone, Debug, Entity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketTags {
    /// The unique identifier of the ticket tags.
    pub id: Key<Self, uuid::Uuid>,

    /// The ticket id.
    pub ticket_id: <Ticket as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,

    /// The timestamp at which the ticket tags was created.
    pub created_at: DateTime<Utc>,
}
