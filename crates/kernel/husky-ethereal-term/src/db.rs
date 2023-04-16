use std::sync::Arc;

use crate::*;
use husky_declarative_ty::DeclarativeTypeDb;
use husky_decr::DecrDb;
use husky_entity_path::EntityPathDb;
use husky_ty_expectation::TypePathDisambiguation;
use salsa::DbWithJar;

pub trait EtherealTermDb: DbWithJar<EtherealTermJar> + DeclarativeTypeDb + DecrDb {
    fn term_menu(&self, toolchain: Toolchain) -> &TermMenu;
    // fn ty_path_ty(
    //     &self,
    //     path: TypePath,
    //     disambiguation: TypePathDisambiguation,
    // ) -> TermResult<EtherealTerm>;
    // fn trai_path_ty(&self, trai_path: TraitPath) -> TermResult<EtherealTerm>;
    // fn form_path_ty(&self, form_path: FormPath) -> TermResult<EtherealTerm>;
    // fn ty_method_card(&self, ty: EtherealTerm, ident: Ident) -> TermResult<Option<TypeMethodFnCard>>;
    // fn field_ty(&self, ty: EtherealTerm, ident: Ident) -> TermResult<Option<EtherealTerm>>;
}

impl<Db> EtherealTermDb for Db
where
    Db: DbWithJar<EtherealTermJar> + DeclarativeTypeDb + DecrDb,
{
    fn term_menu(&self, toolchain: Toolchain) -> &TermMenu {
        term_menu(self, toolchain)
    }

    // fn ty_path_ty(
    //     &self,
    //     path: TypePath,
    //     disambiguation: TypePathDisambiguation,
    // ) -> TermResult<EtherealTerm> {
    //     ty_path_ty(self, path, disambiguation)
    // }

    // fn trai_path_ty(&self, trai_path: TraitPath) -> TermResult<EtherealTerm> {
    //     trai_path_ty_unchecked(self, trai_path)?.checked(self)
    // }

    // fn form_path_ty(&self, form_path: FormPath) -> TermResult<EtherealTerm> {
    //     form_path_ty_unchecked(self, form_path)?.checked(self)
    // }

    // fn ty_method_card(&self, ty: EtherealTerm, ident: Ident) -> TermResult<Option<TypeMethodFnCard>> {
    //     ty_method_card(self, ty, ident)
    // }

    // fn field_ty(&self, ty: EtherealTerm, ident: Ident) -> TermResult<Option<EtherealTerm>> {
    //     field_ty(self, ty, ident)
    // }
}
