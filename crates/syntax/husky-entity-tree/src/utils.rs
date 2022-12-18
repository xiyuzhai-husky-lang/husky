use crate::EntityTreeDb;

pub trait EntityTreeUtils {}

impl<T> EntityTreeUtils for T where T: EntityTreeDb {}
