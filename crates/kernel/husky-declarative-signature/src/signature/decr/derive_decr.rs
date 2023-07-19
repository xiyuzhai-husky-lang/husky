use super::*;
use husky_decr::DeriveDecr;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrDeclarativeSignatureTemplate {
    pub traits: Vec<DeclarativeTerm>,
}

impl HasDeclarativeSignatureTemplate for DeriveDecr {
    type DeclarativeSignatureTemplate = DeriveDecrDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        derive_decr_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn derive_decr_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decr: DeriveDecr,
) -> DeclarativeSignatureResult<DeriveDecrDeclarativeSignatureTemplate> {
    let expr_region = decr.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let traits = decr
        .traits(db)
        .iter()
        .copied()
        .map(|trai_expr| declarative_term_region.expr_term(trai_expr.expr()))
        .collect::<DeclarativeTermResultBorrowed2<Vec<_>>>()?;
    Ok(DeriveDecrDeclarativeSignatureTemplate::new(db, traits))
}

pub trait HasDeriveDecrDeclarativeSignatureTemplates: Copy {
    fn derive_decr_declarative_signature_templates(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<&[DeriveDecrDeclarativeSignatureTemplate]>;
}

impl HasDeriveDecrDeclarativeSignatureTemplates for TypePath {
    fn derive_decr_declarative_signature_templates(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<&[DeriveDecrDeclarativeSignatureTemplate]> {
        Ok(ty_path_derive_decr_declarative_signature_templates(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
fn ty_path_derive_decr_declarative_signature_templates(
    db: &dyn DeclarativeSignatureDb,
    ty_path: TypePath,
) -> DeclarativeSignatureResult<SmallVec<[DeriveDecrDeclarativeSignatureTemplate; 8]>> {
    todo!()
}
