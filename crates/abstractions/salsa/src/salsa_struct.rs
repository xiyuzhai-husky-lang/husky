use crate::*;

pub trait SalsaStructInDb {
    fn register_dependent_fn(db: &Db, index: IngredientIndex);
}

/// A ZST that implements [`SalsaStructInDb`]
///
/// It is used for implementing "constant" tracked function
/// (ones that only take a database as an argument).
pub struct Singleton;

impl SalsaStructInDb for Singleton {
    fn register_dependent_fn(_db: &Db, _index: IngredientIndex) {}
}

impl<I> SalsaStructInDb for I
where
    I: ::eterned::as_id::AsEternedId,
{
    fn register_dependent_fn(db: &Db, index: IngredientIndex) {}
}
