use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::VerifiedTicket;

/// A trait to be implemented by [`VerifiedTickets`]s repositories.
pub trait VerifiedTicketsRepo:
    ReadRepo<Entity = VerifiedTicket> + WriteRepo<Entity = VerifiedTicket>
{
}
