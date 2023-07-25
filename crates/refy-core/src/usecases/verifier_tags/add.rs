use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{
    entities::{Tag, Verifier, VerifierTag},
    error::AppError,
};

/// A use case to add a [`VerifierTag`].
#[derive(Debug, UseCase)]
#[usecase(
    input = "AddVerifierTagInput",
    output = "VerifierTag",
    error = "AppError"
)]
pub struct AddVerifierTag;

#[derive(Debug)]
pub struct AddVerifierTagInput {
    /// The verifier id.
    pub verifier_id: <Verifier as Entity>::Key,

    /// The tag id.
    pub tag_id: <Tag as Entity>::Key,
}
