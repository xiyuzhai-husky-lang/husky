use husky_decr::DecrDb;
use husky_expr::ExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureTemplateJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion;
    fn ty_declarative_signature_template_from_decl(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate>;
    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate>;
    fn ty_item_declarative_signature(
        &self,
        path: TypeItemPath,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate>;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureTemplateJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, expr_region)
    }

    fn ty_declarative_signature_template_from_decl(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
        ty_declarative_signature_template_from_decl(self, decl)
    }

    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate> {
        trai_declarative_signature_from_decl(self, decl)
    }

    fn ty_item_declarative_signature(
        &self,
        path: TypeItemPath,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
        ty_item_declarative_signature(self, path)
    }
}
