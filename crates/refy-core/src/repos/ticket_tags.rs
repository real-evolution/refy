use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::TicketTags;

/// A trait to be implemented by [`TicketTags`]s repositories.
pub trait TicketTagsRepo:
    ReadRepo<Entity = TicketTags> + WriteRepo<Entity = TicketTags>
{
}
