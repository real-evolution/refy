use reddd::domain::{Entity, Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Ticket, TicketTag},
    error::AppError,
};

/// A use case to get a page of [`TicketTag`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "GetTicketTagPageInput",
    output = "Vec<TicketTag>",
    error = "AppError"
)]
pub struct GetTicketTagPage;

#[derive(Debug)]
pub struct GetTicketTagPageInput {
    /// The ticket id.
    pub ticket_id: <Ticket as Entity>::Key,

    /// The tag id.
    pub page: Pagination<TicketTag>,
}
