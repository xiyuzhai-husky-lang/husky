use crate::*;
use husky_raw_term::RawTerm;

pub trait TermDb: salsa::DbWithJar<TermJar> + TermDb {
    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>>;
    fn field_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>>;
    fn reduced_term(&self, term: RawTerm) -> Term;
    fn intrinsic_ty(&self, ty: Term) -> IntrinsicType;
    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion;
    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool;
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TypeResult<Term>;
    fn trai_path_ty_unchecked(&self, trai_path: TraitPath) -> TypeResult<Term>;
    fn form_path_ty_unchecked(&self, form_path: FormPath) -> TypeResult<Term>;
}

impl<Db> TermDb for Db
where
    Db: salsa::DbWithJar<TermJar> + TermDb,
{
    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>> {
        ty_method_ty(self, ty, ident)
    }

    fn reduced_term(&self, term: RawTerm) -> Term {
        todo!()
        // calc_reduced_term(self, term)
    }

    fn field_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>> {
        field_ty(self, ty, ident)
    }

    fn intrinsic_ty(&self, ty: Term) -> IntrinsicType {
        intrinsic_ty(self, ty)
    }

    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion {
        application_expansion(self, reduced_term)
    }

    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool {
        calc_term_symbols(self, term)
            .map(|term_symbols| term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }
}
