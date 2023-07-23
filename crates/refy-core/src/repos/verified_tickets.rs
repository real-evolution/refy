use reddd::domain::{ReadRepo, WriteRepo};

use crate::entities::VerifiedTickets;

/// A trait to be implemented by [`VerifiedTickets`]s repositories.
pub trait VerifiedTicketsRepo:
    ReadRepo<Entity = VerifiedTickets> + WriteRepo<Entity = VerifiedTickets>
{
}
