use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::Ticket;

/// A trait to be implemented by [`Ticket`]s repositories.
pub trait TicketsRepo:
    ReadRepo<Entity = Ticket> + WriteRepo<Entity = Ticket>
{
}
