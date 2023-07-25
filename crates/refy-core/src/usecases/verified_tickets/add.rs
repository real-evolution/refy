use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Tag, Ticket, VerifiedTicket, Verifier},
    error::AppError,
};

/// A use case to add a [`VerifiedTicket`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "AddVerifiedTicketInput",
    output = "VerifiedTicket",
    error = "AppError"
)]
pub struct AddVerifiedTicket;

#[derive(Debug)]
pub struct AddVerifiedTicketInput {
    /// The note of the struct.
    pub note: String,

    /// The verifier id.
    pub verifier_id: <Verifier as Entity>::Key,

    /// The ticket id.
    pub ticket_id: <Ticket as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,
}
