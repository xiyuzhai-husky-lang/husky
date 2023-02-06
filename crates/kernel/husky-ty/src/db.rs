use crate::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, path: EntityPath) -> TypeResult<Term>;
    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>>;
    fn term_ty(&self, term: Term) -> TypeResult<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, path: EntityPath) -> TypeResult<Term> {
        entity_ty(self, path)
    }

    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>> {
        ty_method_ty(self, reduced_term(self, ty), ident)
    }
    fn term_ty(&self, term: Term) -> TypeResult<Term> {
        term_ty(self, term)
    }
}
