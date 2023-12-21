use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct ValFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
    pub return_ty: EtherealTerm,
}

impl ValFugitiveEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        declarative_signature_template: ValFugitiveDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let return_ty = EtherealTerm::ty_from_declarative(
            db,
            declarative_signature_template.initialization_ty(db),
        )?;
        Ok(Self::new(db, path, return_ty))
    }
}
