use crate::IsLinkageImpl;
use std::any::{Any, TypeId};

pub struct AnyLinkageImpls {
    linkage_impls: Box<dyn Any>,
    /// the type id of `LinkageImpl`
    type_id: TypeId,
}

impl AnyLinkageImpls {
    pub fn new<LinkageImpl: IsLinkageImpl>(linkage_impls: Vec<LinkageImpl>) -> Self {
        Self {
            linkage_impls: Box::new(linkage_impls),
            type_id: TypeId::of::<LinkageImpl>(),
        }
    }

    pub fn downcast<LinkageImpl: IsLinkageImpl>(self) -> Vec<LinkageImpl> {
        assert_eq!(self.type_id, TypeId::of::<LinkageImpl>());
        *self.linkage_impls.downcast().unwrap()
    }
}
