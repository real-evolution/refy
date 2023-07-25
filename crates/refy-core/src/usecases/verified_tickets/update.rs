use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifiedTicket, error::AppError};

/// A use case to update a [`VerifiedTicket`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "UpdateVerifiedTicketInput",
    output = "VerifiedTicket",
    error = "AppError"
)]
pub struct UpdateVerifiedTicket;

#[derive(Debug)]
pub struct UpdateVerifiedTicketInput {
    /// The unique identifier of the verified ticket.
    pub id: <VerifiedTicket as Entity>::Key,

    /// The note of the verification.
    pub note: String,
}
