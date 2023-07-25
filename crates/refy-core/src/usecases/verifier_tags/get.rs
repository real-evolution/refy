use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifierTag, error::AppError};

/// A use case to get a [`VerifierTag`] by its id.
#[derive(Debug, UseCase)]
#[usecase(
    input = "<VerifierTag as Entity>::Key",
    output = "VerifierTag",
    error = "AppError"
)]
pub struct GetVerifierTag;
