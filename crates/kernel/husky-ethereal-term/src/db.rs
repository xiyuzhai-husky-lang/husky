use std::sync::Arc;

use crate::*;
use husky_declarative_ty::DeclarativeTypeDb;
use husky_entity_path::EntityPathDb;
use salsa::DbWithJar;

pub trait EtherealTermDb: DbWithJar<EtherealTermJar> + DeclarativeTypeDb {
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu;
    // fn ty_path_ty(
    //     &self,
    //     path: TypePath,
    //     disambiguation: TypePathDisambiguation,
    // ) -> EtherealTermResult<EtherealTerm>;
    // fn trai_path_ty(&self, trai_path: TraitPath) -> EtherealTermResult<EtherealTerm>;
    // fn form_path_ty(&self, form_path: FugitivePath) -> EtherealTermResult<EtherealTerm>;
    // fn ty_method_card(&self, ty: EtherealTerm, ident: Ident) -> EtherealTermResult<Option<TypeMethodFnCard>>;
    // fn field_ty(&self, ty: EtherealTerm, ident: Ident) -> EtherealTermResult<Option<EtherealTerm>>;
}

impl<Db> EtherealTermDb for Db
where
    Db: DbWithJar<EtherealTermJar> + DeclarativeTypeDb,
{
    fn ethereal_term_menu(&self, toolchain: Toolchain) -> &EtherealTermMenu {
        term_menu(self, toolchain)
    }

    // fn ty_path_ty(
    //     &self,
    //     path: TypePath,
    //     disambiguation: TypePathDisambiguation,
    // ) -> EtherealTermResult<EtherealTerm> {
    //     ty_path_ty(self, path, disambiguation)
    // }

    // fn trai_path_ty(&self, trai_path: TraitPath) -> EtherealTermResult<EtherealTerm> {
    //     trai_path_ty_unchecked(self, trai_path)?.checked(self)
    // }

    // fn form_path_ty(&self, form_path: FugitivePath) -> EtherealTermResult<EtherealTerm> {
    //     form_path_ty_unchecked(self, form_path)?.checked(self)
    // }

    // fn ty_method_card(&self, ty: EtherealTerm, ident: Ident) -> EtherealTermResult<Option<TypeMethodFnCard>> {
    //     ty_method_card(self, ty, ident)
    // }

    // fn field_ty(&self, ty: EtherealTerm, ident: Ident) -> EtherealTermResult<Option<EtherealTerm>> {
    //     field_ty(self, ty, ident)
    // }
}
