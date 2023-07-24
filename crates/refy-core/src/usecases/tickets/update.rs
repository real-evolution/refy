use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Ticket, error::AppError};

/// A use case to update a [`Ticket`].
#[derive(Debug, UseCase)]
#[usecase(input = "UpdateTicketInput", output = "Ticket", error = "AppError")]
pub struct UpdateTicket;

#[derive(Debug)]
pub struct UpdateTicketInput {
    /// The unique identifier of the tag.
    pub id: <Ticket as Entity>::Key,

    /// The title of the ticket.
    pub title: String,
}
