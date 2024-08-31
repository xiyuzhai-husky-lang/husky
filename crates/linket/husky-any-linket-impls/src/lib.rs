#![feature(downcast_unchecked)]
use colored::Colorize;
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{eval_context::DevEvalContextGuard, linket_impls::LinketImpls};
use husky_linket_impl::{
    eval_context::{DevEvalContext, IsDevRuntimeDyn},
    linket_impl::IsLinketImpl,
};
use std::any::Any;

pub struct AnyLinketImpls {
    linket_impls: Box<dyn Any>,
    rustc_version: rustc_version::Version,
    linket_impl_type_name: &'static str,
}

pub type LinketImplsGetter = extern "C" fn(&[Option<ItemPathIdInterface>]) -> AnyLinketImpls;

pub static LINKET_IMPLS_GETTER_IDENT: &[u8] = b"linket_impls";

impl AnyLinketImpls {
    pub fn new<DevsoulInterface: IsDevsoulInterface>(
        linket_impls: Vec<DevsoulInterface::LinketImpl>,
    ) -> Self {
        Self {
            linket_impls: Box::new(LinketImpls::new(
                DevsoulInterface::try_set_dev_eval_context,
                linket_impls,
            )),
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

/// generates the function to acquire linket impls accessed through dynamic library,
///
/// it also set up the jar index.
#[macro_export]
macro_rules! linket_impls {
    ($($linket_impl: expr),* $(,)?) => {
        #[no_mangle]
        pub extern "C" fn linket_impls(item_path_id_interfaces: &[Option<__ItemPathIdInterface>]) -> AnyLinketImpls {
            let linkets: Vec<__LinketImpl> =
                vec![
                    $($linket_impl),*
                ];
            for (&item_path_id_interface, &linket) in std::iter::zip(item_path_id_interfaces,&linkets) {
                if let Some(item_path_id_interface) = item_path_id_interface {
                    linket.init_item_path_id_interface(item_path_id_interface)
                }
            }
            AnyLinketImpls::new::<__DevsoulInterface>(linkets)
        }
    };
}

#[test]
fn linket_impls_works() {
    use husky_devsoul_interface::ugly::*;
    use husky_standard_devsoul_interface::ugly::*;
    use husky_standard_linket_impl::{ugly::*, *};

    type __LinketImpl = StandardLinketImpl;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinketImpl = __LinketImpl;

        fn dev_eval_context() -> DevEvalContext<Self::LinketImpl> {
            todo!()
        }

        fn try_set_dev_eval_context(
            ctx: DevEvalContext<Self::LinketImpl>,
        ) -> Result<DevEvalContextGuard, ()> {
            todo!()
        }
    }

    linket_impls! {}

    linket_impls as LinketImplsGetter;
    linket_impls(&[]);
}
