use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifiedTicket, error::AppError};

/// A use case to remove a [`VerifiedTicket`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "<VerifiedTicket as Entity>::Key",
    output = "()",
    error = "AppError"
)]
pub struct RemoveVerifiedTicket;
