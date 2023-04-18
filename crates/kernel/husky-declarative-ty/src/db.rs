use crate::*;

pub trait DeclarativeTypeDb: salsa::DbWithJar<DeclarativeTypeJar> + DeclarativeSignatureDb {}

impl<Db> DeclarativeTypeDb for Db where
    Db: salsa::DbWithJar<DeclarativeTypeJar> + DeclarativeSignatureDb
{
}
