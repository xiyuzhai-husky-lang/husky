mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum MajorItemEtherealSignatureTemplate {
    Type(TypeEtherealSignatureTemplate),
    Fugitive(FugitiveEtherealSignatureTemplate),
    Trait(TraitEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for MajorItemPath {
    type EtherealSignatureTemplate = MajorItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        Ok(match self {
            MajorItemPath::Type(path) => path.ethereal_signature_template(db)?.into(),
            MajorItemPath::Trait(path) => path.ethereal_signature_template(db)?.into(),
            MajorItemPath::Fugitive(path) => path.ethereal_signature_template(db)?.into(),
        })
    }
}
