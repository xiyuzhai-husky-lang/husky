use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumUnitTypeVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
    pub self_ty: EtherealTerm,
}

impl EnumUnitTypeVariantEtherealSignatureTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.self_ty(db)
    }

    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypeVariantPath,
        tmpl: EnumUnitTypeVariantDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEtherealSignatureTemplate::Enum(parent_ty_template) =
            path.parent_ty_path(db).ethereal_signature_template(db)?
        else {
            unreachable!()
        };
        let self_ty = EtherealTerm::ty_from_declarative(db, tmpl.self_ty(db))?;
        Ok(Self::new(db, parent_ty_template, self_ty))
    }
}
