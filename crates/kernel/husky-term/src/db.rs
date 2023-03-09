use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_raw_ty::RawTypeDb;
use husky_ty_expectation::TypePathDisambiguation;
use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + RawTypeDb {
    fn term_menu(&self, toolchain: Toolchain) -> TermResult<&TermMenu>;
    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion;
    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool;
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<Term>;
    fn trai_path_ty(&self, trai_path: TraitPath) -> TermResult<Term>;
    fn form_path_ty(&self, form_path: FormPath) -> TermResult<Term>;
    fn ty_method_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>>;
    fn field_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>>;
}

impl<Db> TermDb for Db
where
    Db: DbWithJar<TermJar> + RawTypeDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> TermResult<&TermMenu> {
        term_menu(self, toolchain).as_ref().map_err(|e| *e)
    }

    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion {
        application_expansion(self, reduced_term)
    }

    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool {
        calc_term_symbols(self, term)
            .map(|term_symbols| term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }

    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<Term> {
        ty_path_ty(self, path, disambiguation)
    }

    fn trai_path_ty(&self, trai_path: TraitPath) -> TermResult<Term> {
        trai_path_ty_unchecked(self, trai_path)?.checked(self)
    }

    fn form_path_ty(&self, form_path: FormPath) -> TermResult<Term> {
        form_path_ty_unchecked(self, form_path)?.checked(self)
    }

    fn ty_method_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>> {
        ty_method_ty(self, ty, ident)
    }

    fn field_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>> {
        field_ty(self, ty, ident)
    }
}
