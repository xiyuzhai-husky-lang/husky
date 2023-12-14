use colored::Colorize;

use crate::IsLinkageImpl;
use std::any::{Any, TypeId};

pub struct AnyLinkageImpls {
    linkage_impls: Box<dyn Any>,
    rustc_version: rustc_version::Version,
    linkage_impl_type_name: &'static str,
}

impl AnyLinkageImpls {
    pub fn new<LinkageImpl: IsLinkageImpl>(linkage_impls: Vec<LinkageImpl>) -> Self {
        Self {
            linkage_impls: Box::new(linkage_impls),
            rustc_version: rustc_version::version().unwrap(),
            linkage_impl_type_name: std::any::type_name::<LinkageImpl>(),
        }
    }

    pub fn downcast<LinkageImpl: IsLinkageImpl>(self) -> Vec<LinkageImpl> {
        if !self.is_same_ty::<LinkageImpl>() {
            panic!(
                r#"expected `{}` compiled with rustc version {},
but get `{}` compiled with rustc version {} instead."#,
                std::any::type_name::<LinkageImpl>().blue(),
                self.rustc_version,
                self.linkage_impl_type_name.red(),
                rustc_version::version().unwrap()
            )
        }
        *unsafe { self.linkage_impls.downcast_unchecked() }
    }

    fn is_same_ty<LinkageImpl: IsLinkageImpl>(&self) -> bool {
        // type id doesn't really work
        // self.type_id != TypeId::of::<LinkageImpl>()
        self.linkage_impl_type_name == std::any::type_name::<LinkageImpl>()
            && self.rustc_version == rustc_version::version().unwrap()
    }
}
