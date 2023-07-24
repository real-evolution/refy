use reddd::domain::{Entity, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Tag, error::AppError};

/// A use case to update a [`Tag`].
#[derive(Debug, UseCase)]
#[usecase(input = "UpdateTagInput", output = "Tag", error = "AppError")]
pub struct UpdateTag;

#[derive(Debug)]
pub struct UpdateTagInput {
    /// The unique identifier of the tag.
    pub id: <Tag as Entity>::Key,

    /// The code of the tag.
    pub tag: String,
}
