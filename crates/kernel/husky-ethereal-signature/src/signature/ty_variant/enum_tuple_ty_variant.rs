use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumTupleVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
    pub instance_constructor_ritchie_ty: EtherealTermRitchie,
}

impl EnumTupleVariantEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypeVariantPath,
        tmpl: EnumTupleVariantDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEtherealSignatureTemplate::Enum(parent_ty_template) =
            path.parent_ty_path(db).ethereal_signature_template(db)?
        else {
            unreachable!()
        };
        let instance_constructor_ty =
            EtherealTermRitchie::from_declarative(db, tmpl.instance_constructor_ty(db))?;
        Ok(Self::new(db, parent_ty_template, instance_constructor_ty))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
