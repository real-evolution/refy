use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::VerifierTags;

/// A trait to be implemented by [`VerifierTags`]s repositories.
pub trait VerifierTagsRepo:
    ReadRepo<Entity = VerifierTags> + WriteRepo<Entity = VerifierTags>
{
}
