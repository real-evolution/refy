use reddd::domain::{Entity, Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Verifier, VerifierTag},
    error::AppError,
};

/// A use case to get a page of [`VerifierTag`]s.
#[derive(Debug, UseCase)]
#[usecase(
    input = "GetVerifierTagPageInput",
    output = "Vec<VerifierTag>",
    error = "AppError"
)]
pub struct GetVerifierTagPage;

#[derive(Debug)]
pub struct GetVerifierTagPageInput {
    /// The verifier id.
    pub verifier_id: <Verifier as Entity>::Key,

    /// The tag id.
    pub page: Pagination<VerifierTag>,
}
