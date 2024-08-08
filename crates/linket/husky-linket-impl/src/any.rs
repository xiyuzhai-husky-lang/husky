use crate::{IsLinketImpl, LinketImpls};
use colored::Colorize;
use husky_devsoul_interface::{
    devsoul::IsDevsoulInterface, item_path::ItemPathIdInterface, DevEvalContext,
};
use std::any::Any;

pub struct AnyLinketImpls {
    linket_impls: Box<dyn Any>,
    rustc_version: rustc_version::Version,
    linket_impl_type_name: &'static str,
}

impl AnyLinketImpls {
    pub fn new<DevsoulInterface: IsDevsoulInterface>(
        linket_impls: Vec<DevsoulInterface::LinketImpl>,
    ) -> Self {
        Self {
            linket_impls: Box::new(LinketImpls {
                set_dev_eval_context: DevsoulInterface::set_dev_eval_context,
                unset_dev_eval_context: DevsoulInterface::unset_dev_eval_context,
                init_item_path_id_interface_caches: |_| todo!(),
                linket_impls,
            }),
            rustc_version: rustc_version::version().unwrap(),
            linket_impl_type_name: std::any::type_name::<DevsoulInterface::LinketImpl>(),
        }
    }

    /// a stable downcast method instead of the unstable builtin one using typeids
    pub fn downcast<LinketImpl: IsLinketImpl>(self) -> LinketImpls<LinketImpl> {
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
