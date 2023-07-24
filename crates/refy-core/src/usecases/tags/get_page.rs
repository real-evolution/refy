use reddd::domain::{Pagination, UseCase};
use reddd_macros::UseCase;

use crate::{entities::Tag, error::AppError};

/// A use case to get a page of [`Tag`]s.
#[derive(Debug, UseCase)]
#[usecase(input = "Pagination<Tag>", output = "Vec<Tag>", error = "AppError")]
pub struct GetTagPage {}
