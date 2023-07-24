use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Tag, error::AppError};

/// A use case to get a [`Tag`] by its id.
#[derive(Debug, UseCase)]
#[usecase(input = "<Tag as Entity>::Key", output = "Tag", error = "AppError")]
pub struct GetTag;
