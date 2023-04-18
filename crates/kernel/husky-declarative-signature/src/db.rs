use husky_decr::DecrDb;
use husky_expr::ExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion;
    fn ty_declarative_signature_from_decl(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignature>;
    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignature>;
    fn ty_item_declarative_signature(
        &self,
        path: TypeItemPath,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature>;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, expr_region)
    }

    fn ty_declarative_signature_from_decl(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignature> {
        ty_declarative_signature_from_decl(self, decl)
    }

    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignature> {
        trai_declarative_signature_from_decl(self, decl)
    }

    fn ty_item_declarative_signature(
        &self,
        path: TypeItemPath,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature> {
        ty_item_declarative_signature(self, path)
    }
}
