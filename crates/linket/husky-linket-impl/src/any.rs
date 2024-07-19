use colored::Colorize;

use crate::IsLinketImpl;
use std::any::Any;

pub struct AnyLinketImpls {
    linket_impls: Box<dyn Any>,
    rustc_version: rustc_version::Version,
    linket_impl_type_name: &'static str,
}

impl AnyLinketImpls {
    pub fn new<LinketImpl: IsLinketImpl>(linket_impls: Vec<LinketImpl>) -> Self {
        Self {
            linket_impls: Box::new(linket_impls),
            rustc_version: rustc_version::version().unwrap(),
            linket_impl_type_name: std::any::type_name::<LinketImpl>(),
        }
    }

    pub fn downcast<LinketImpl: IsLinketImpl>(self) -> Vec<LinketImpl> {
        if !self.is_same_ty::<LinketImpl>() {
            panic!(
                r#"expected `{}` compiled with rustc version {},
but get `{}` compiled with rustc version {} instead."#,
                std::any::type_name::<LinketImpl>().blue(),
                self.rustc_version,
                self.linket_impl_type_name.red(),
                rustc_version::version().unwrap()
            )
        }
        *unsafe { self.linket_impls.downcast_unchecked() }
    }

    fn is_same_ty<LinketImpl: IsLinketImpl>(&self) -> bool {
        // type id doesn't really work
        // self.type_id != TypeId::of::<LinketImpl>()
        self.linket_impl_type_name == std::any::type_name::<LinketImpl>()
            && self.rustc_version == rustc_version::version().unwrap()
    }
}
