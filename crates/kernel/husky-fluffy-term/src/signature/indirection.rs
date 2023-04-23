use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyIndirection {
    Unleash,
    Deref { lifetime: FluffyTerm },
    DerefMut { lifetime: FluffyTerm },
    Place(Place),
}
