use std::sync::Arc;

use crate::*;
use husky_decr::DecrDb;
use husky_entity_path::EntityPathDb;
use husky_raw_ty::RawTypeDb;
use husky_ty_expectation::TypePathDisambiguation;
use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> + RawTypeDb + DecrDb {
    fn term_menu(&self, toolchain: Toolchain) -> &TermMenu;
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<Term>;
    fn trai_path_ty(&self, trai_path: TraitPath) -> TermResult<Term>;
    fn form_path_ty(&self, form_path: FormPath) -> TermResult<Term>;
    // fn ty_method_card(&self, ty: Term, ident: Ident) -> TermResult<Option<TypeMethodFnCard>>;
    // fn field_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>>;
}

impl<Db> TermDb for Db
where
    Db: DbWithJar<TermJar> + RawTypeDb + DecrDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> &TermMenu {
        term_menu(self, toolchain)
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

    // fn ty_method_card(&self, ty: Term, ident: Ident) -> TermResult<Option<TypeMethodFnCard>> {
    //     ty_method_card(self, ty, ident)
    // }

    // fn field_ty(&self, ty: Term, ident: Ident) -> TermResult<Option<Term>> {
    //     field_ty(self, ty, ident)
    // }
}
