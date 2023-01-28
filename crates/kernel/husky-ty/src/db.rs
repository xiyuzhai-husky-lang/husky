use crate::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, path: EntityPath) -> TypeResultRef<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, path: EntityPath) -> TypeResultRef<Term> {
        entity_ty(self, path)
    }
}
