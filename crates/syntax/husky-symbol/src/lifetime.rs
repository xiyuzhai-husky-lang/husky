use crate::*;

pub struct Lifetime;

pub type LifetimeArena = Arena<Lifetime>;
pub type LifetimeIdx = ArenaIdx<Lifetime>;
