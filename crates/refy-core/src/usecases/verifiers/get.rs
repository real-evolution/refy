use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Verifier, error::AppError};

/// A use case to get a [`Verifier`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<Verifier as Entity>::Key",
    output = "Verifier",
    error = "AppError"
)]
pub struct GetVerifier;
