use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::VerifierTag, error::AppError};

/// A use case to remove a [`VerifierTag`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "<VerifierTag as Entity>::Key",
    output = "()",
    error = "AppError"
)]
pub struct RemoveVerifierTag;
