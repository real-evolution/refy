use reddd::domain::{Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Ticket, error::AppError};

/// A use case to get a page of [`Ticket`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "Pagination<Ticket>",
    output = "Vec<Ticket>",
    error = "AppError"
)]
pub struct GetTicketPage;
