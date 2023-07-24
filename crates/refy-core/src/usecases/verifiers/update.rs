use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Verifier, error::AppError};

/// A use case to update a [`Verifier`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "UpdateVerifierInput",
    output = "Verifier",
    error = "AppError"
)]
pub struct UpdateVerifier;

#[derive(Debug)]
pub struct UpdateVerifierInput {
    /// The unique identifier of the Verifier.
    pub id: <Verifier as Entity>::Key,

    /// The display name of the verifier.
    pub display_name: String,
}
