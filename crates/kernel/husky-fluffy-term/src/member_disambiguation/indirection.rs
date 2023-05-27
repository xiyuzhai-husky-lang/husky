use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyIndirection {
    Place(Place),
    Leash,
}
