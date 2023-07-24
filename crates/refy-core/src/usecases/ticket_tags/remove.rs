use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::TicketTag, error::AppError};

/// A use case to remove a [`TicketTag`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "<TicketTag as Entity>::Key",
    output = "()",
    error = "AppError"
)]
pub struct RemoveTicketTag;
