use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Verifier, error::AppError};

/// A use case to remove a [`Verifier`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "<Verifier as Entity>::Key",
    output = "()",
    error = "AppError"
)]
pub struct RemoveVerifier;
