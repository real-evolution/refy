use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::Verifier;

/// A trait to be implemented by [`Verifier`]s repositories.
pub trait VerifiersRepo:
    ReadRepo<Entity = Verifier> + WriteRepo<Entity = Verifier>
{
}
