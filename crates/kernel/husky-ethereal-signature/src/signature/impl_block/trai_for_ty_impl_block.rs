use super::*;
use smallvec::SmallVec;

pub struct TraitForTypeImplBlockSignatureTemplate {}

pub type TraitForTypeImplBlockSignatureTemplates =
    SmallVec<[TraitForTypeImplBlockSignatureTemplate; 2]>;

impl TraitForTypeImplBlockSignatureTemplate {
    pub fn collect_from_ty_side(db: &dyn EtherealSignatureDb) -> SmallVec<[Self; 2]> {
        todo!()
    }
}
