use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Tag, Ticket, TicketTag},
    error::AppError,
};

/// A use case to add a [`TicketTag`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "AddTicketTagInput",
    output = "TicketTag",
    error = "AppError"
)]
pub struct AddTicketTag;

#[derive(Debug)]
pub struct AddTicketTagInput {
    /// The ticket id.
    pub ticket_id: <Ticket as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,
}
