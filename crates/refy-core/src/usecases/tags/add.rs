use reddd::domain::UseCase;
use reddd_macros::UseCase;

use crate::{entities::Tag, error::AppError};

/// A use case to add a [`Tag`].
#[derive(Debug, UseCase)]
#[usecase(input = "String", output = "Tag", error = "AppError")]
pub struct AddTag;
