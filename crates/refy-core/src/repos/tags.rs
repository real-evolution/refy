use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::Tag;

/// A trait to be implemented by [`Tag`]s repositories.
pub trait TagsRepo: ReadRepo<Entity = Tag> + WriteRepo<Entity = Tag> {}
