use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Ticket, error::AppError};

/// A use case to remove a [`Ticket`].
#[derive(Debug, UseCase)]
#[usecase(input = "<Ticket as Entity>::Key", output = "()", error = "AppError")]
pub struct RemoveTicket;
