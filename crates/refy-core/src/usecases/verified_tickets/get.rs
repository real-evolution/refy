use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifiedTicket, error::AppError};

/// A use case to get a [`VerifiedTicket`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<VerifiedTicket as Entity>::Key",
    output = "VerifiedTicket",
    error = "AppError"
)]
pub struct GetVerifiedTicket;
