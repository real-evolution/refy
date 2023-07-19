use chrono::{DateTime, Utc};
use reddd::domain::{Entity, Key, MutableEntity};
use reddd_macros::MutableEntity;
use serde::{Deserialize, Serialize};

/// A struct representing a ticket.
#[derive(Clone, Debug, MutableEntity, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    /// The unique identifier of the ticket.
    pub id: Key<Self, uuid::Uuid>,

    /// The title of the ticket.
    pub title: String,

    /// The timestamp at which the ticket was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp that the latest modification was made.
    pub updated_at: DateTime<Utc>,
}
