use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::TicketTag;

/// A trait to be implemented by [`TicketTags`]s repositories.
pub trait TicketTagsRepo:
    ReadRepo<Entity = TicketTag> + WriteRepo<Entity = TicketTag>
{
}
