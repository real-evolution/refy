use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::TicketTag, error::AppError};

/// A use case to get a [`TicketTag`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<TicketTag as Entity>::Key",
    output = "TicketTag",
    error = "AppError"
)]
pub struct GetTicketTag;
