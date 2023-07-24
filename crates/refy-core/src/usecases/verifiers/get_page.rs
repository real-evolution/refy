use reddd::domain::{Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Verifier, error::AppError};

/// A use case to get a page of [`Verifier`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "Pagination<Verifier>",
    output = "Vec<Verifier>",
    error = "AppError"
)]
pub struct GetVeriferiPage {}
