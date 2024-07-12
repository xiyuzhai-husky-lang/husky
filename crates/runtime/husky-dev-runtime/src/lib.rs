#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_linkage::linkage::Linkage;
use husky_task::{
    dev_ascension::IsDevAscension,
    helpers::{DevAscensionException, DevAscensionValue},
};
use husky_task::{
    dev_ascension::IsRuntimeStorage,
    helpers::{DevAscensionKiControlFlow, DevAscensionValueResult},
};
use husky_task_interface::{
    ki_repr::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface},
    HuskyIngredientIndex, HuskyJarIndex, IsDevRuntime, IsLinkageImpl, LinkageImplKiControlFlow,
};
use husky_vfs::{error::VfsResult, path::linktime_target_path::LinktimeTargetPath};
use std::{convert::Infallible, path::Path};

/// Dropping libraries or linkage_impls before runtime storage will lead to segmentation fault
///
/// so it's necessary to pub `storage` field before `comptime`
pub struct DevRuntime<DevAscension: IsDevAscension> {
    config: DevRuntimeConfig<DevAscension>,
    storage: DevAscension::RuntimeStorage,
    comptime: DevComptime<DevAscension>,
}

impl<DevAscension: IsDevAscension> DevRuntime<DevAscension> {
    pub fn new(
        target_crate: impl AsRef<Path>,
        config: Option<DevRuntimeConfig<DevAscension>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate)?,
        })
    }

    pub fn db(&self) -> &::salsa::Db {
        self.comptime.db()
    }

    pub fn comptime_target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.comptime.linktime_target_path()
    }
}

impl<DevAscension: IsDevAscension> Default for DevRuntime<DevAscension>
where
    DevAscension::Linktime: Default,
{
    fn default() -> Self {
        Self {
            comptime: Default::default(),
            storage: Default::default(),
            config: Default::default(),
        }
    }
}

impl<DevAscension: IsDevAscension> IsDevRuntime<DevAscension::LinkageImpl>
    for DevRuntime<DevAscension>
{
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_ingredient_at_pedestal_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        base_point: DevAscension::Pedestal,
        f: impl FnOnce() -> DevAscensionValueResult<DevAscension>,
    ) -> DevAscensionKiControlFlow<DevAscension> {
        self.storage.get_or_try_init_val_value(
            self.comptime.ingredient_val(jar_index, ingredient_index),
            base_point,
            f,
            self.db(),
        )
    }

    fn eval_ingredient_at_pedestal(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: DevAscension::Pedestal,
    ) -> DevAscensionKiControlFlow<DevAscension> {
        self.eval_ki_repr_at_pedestal(
            self.comptime
                .ingredient_ki_repr(jar_index, ingredient_index),
            pedestal,
        )
    }

    fn eval_ki_repr_interface_at_pedestal(
        &self,
        ki_repr_interface: KiReprInterface,
        pedestal: DevAscension::Pedestal,
    ) -> DevAscensionKiControlFlow<DevAscension> {
        self.eval_ki_repr_at_pedestal(ki_repr_interface.into(), pedestal)
    }

    fn eval_ki_domain_repr_interface_at_pedestal(
        &self,
        ki_domain_repr: KiDomainReprInterface,
        pedestal: DevAscension::Pedestal,
    ) -> husky_task_interface::ki_control_flow::KiControlFlow<
        (),
        Infallible,
        DevAscensionException<DevAscension>,
    > {
        self.eval_ki_domain_repr_at_pedestal(ki_domain_repr.into(), pedestal)
    }

    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        pedestal: DevAscension::Pedestal,
        f: impl FnOnce(KiDomainReprInterface) -> DevAscensionKiControlFlow<DevAscension>,
    ) -> DevAscensionKiControlFlow<DevAscension> {
        let db = self.db();
        let ki_repr: KiRepr = unsafe { std::mem::transmute(ki_repr) };
        let ki_domain_repr: KiDomainReprInterface =
            unsafe { std::mem::transmute(ki_repr.ki_domain_repr(db)) };
        self.storage
            .get_or_try_init_val_value(ki_repr.val(db), pedestal, || f(ki_domain_repr), db)
    }

    fn eval_memo_field_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: DevAscension::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> DevAscensionKiControlFlow<DevAscension>,
    ) -> DevAscensionKiControlFlow<DevAscension> {
        self.storage
            .get_or_try_init_memo_field_value(jar_index, ingredient_index, pedestal, slf, f)
    }

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> DevAscensionValue<DevAscension> {
        use husky_value_interface::IsValue;

        let db = self.db();
        let val_runtime_constant: KiRuntimeConstant =
            unsafe { std::mem::transmute(val_runtime_constant) };
        match val_runtime_constant.data(db) {
            KiRuntimeConstantData::TypeVariantPath(path) => {
                let presenter = self
                    .comptime
                    .linkage_impl(Linkage::new_enum_index_presenter(
                        path.parent_ty_path(db),
                        db,
                    ))
                    .enum_index_value_presenter();
                DevAscensionValue::<DevAscension>::from_enum_index(path.index(db).raw(), presenter)
            }
        }
    }
}
