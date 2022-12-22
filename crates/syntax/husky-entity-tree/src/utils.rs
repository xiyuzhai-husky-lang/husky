use crate::EntityTreeDb;

pub trait EntitySymbolUtils {}

impl<T> EntitySymbolUtils for T where T: EntityTreeDb {}
