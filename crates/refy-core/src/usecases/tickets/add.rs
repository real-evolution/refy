use reddd::domain::UseCase;
use reddd_macros::UseCase;

use crate::{entities::Ticket, error::AppError};

/// A use case to add a [`Ticket`].
#[derive(Debug, UseCase)]
#[usecase(input = "String", output = "Ticket", error = "AppError")]
pub struct AddTicket;
