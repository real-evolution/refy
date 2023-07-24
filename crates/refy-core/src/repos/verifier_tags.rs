use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::VerifierTag;

/// A trait to be implemented by [`VerifierTags`]s repositories.
pub trait VerifierTagsRepo:
    ReadRepo<Entity = VerifierTag> + WriteRepo<Entity = VerifierTag>
{
}
