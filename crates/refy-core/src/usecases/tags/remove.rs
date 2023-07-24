use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Tag, error::AppError};

/// A use case to remove a [`Tag`].
#[derive(Debug, UseCase)]
#[usecase(input = "<Tag as Entity>::Key", output = "()", error = "AppError")]
pub struct RemoveTag;
