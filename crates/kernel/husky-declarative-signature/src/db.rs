use husky_decr::DecrDb;
use husky_expr::ExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion;
    fn ty_declarative_signature_template(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate>;
    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate>;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, expr_region)
    }

    fn ty_declarative_signature_template(
        &self,
        decl: TypeDecl,
    ) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
        ty_declarative_signature_template(self, decl)
    }

    fn trai_declarative_signature(
        &self,
        decl: TraitDecl,
    ) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate> {
        trai_declarative_signature_template(self, decl)
    }
}
