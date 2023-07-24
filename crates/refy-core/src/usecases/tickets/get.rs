use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Ticket, error::AppError};

/// A use case to get a [`Ticket`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<Ticket as Entity>::Key",
    output = "Ticket",
    error = "AppError"
)]
pub struct GetTicket;
