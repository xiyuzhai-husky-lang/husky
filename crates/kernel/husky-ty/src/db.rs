use crate::*;
use husky_vfs::Toolchain;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, path: EntityPath) -> TypeResult<ReducedTerm>;
    fn ty_method_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>>;
    fn field_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>>;
    fn term_ty(&self, term: Term) -> TypeResult<ReducedTerm>;
    fn reduced_term(&self, term: Term) -> ReducedTerm;
    fn reduced_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedTermMenu<'a>, &'a TermError>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, path: EntityPath) -> TypeResult<ReducedTerm> {
        entity_ty(self, path)
    }

    fn ty_method_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>> {
        ty_method_ty(self, ty, ident)
    }

    fn term_ty(&self, term: Term) -> TypeResult<ReducedTerm> {
        term_ty(self, term)
    }

    fn reduced_term(&self, term: Term) -> ReducedTerm {
        reduced_term(self, term)
    }

    fn reduced_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedTermMenu<'a>, &'a TermError> {
        let term_menu = self.term_menu(toolchain).as_ref()?;
        Ok(ReducedTermMenu::new(term_menu))
    }

    fn field_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>> {
        field_ty(self, ty, ident)
    }
}
