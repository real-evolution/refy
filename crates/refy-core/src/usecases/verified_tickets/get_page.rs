use reddd::domain::{Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifiedTicket, error::AppError};

/// A use case to get a page of [`VerifiedTicket`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "Pagination<VerifiedTicket>",
    output = "Vec<VerifiedTicket>",
    error = "AppError"
)]
pub struct GetVerifiedTicketPage;
