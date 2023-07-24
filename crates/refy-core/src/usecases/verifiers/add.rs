use reddd::domain::UseCase;
use reddd_macros::UseCase;

use crate::{entities::Verifier, error::AppError};

/// A use case to add a [`Verifier`].
#[derive(Debug, UseCase)]
#[usecase(input = "AddVerifierInput", output = "Verifier", error = "AppError")]
pub struct AddVerifier;

#[derive(Debug)]
pub struct AddVerifierInput {
    /// The display name of the verifier.
    pub display_name: String,

    /// The unique identifier of the owner of the verifier.
    pub principle_id: uuid::Uuid,
}
