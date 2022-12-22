use crate::EntitySymbolDb;

pub trait EntitySymbolUtils {}

impl<T> EntitySymbolUtils for T where T: EntitySymbolDb {}
