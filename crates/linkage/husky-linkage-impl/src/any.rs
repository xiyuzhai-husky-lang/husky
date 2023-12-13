use crate::IsLinkageImpl;

pub struct AnyLinkageImpls {
    linkage_impls: Vec<()>,
    type_id: std::any::TypeId,
}

impl AnyLinkageImpls {
    pub fn downcast<LinkageImpl: IsLinkageImpl>(self) -> Vec<LinkageImpl> {
        todo!()
    }
}
