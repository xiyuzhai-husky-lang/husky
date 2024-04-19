#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_entity_path::TypeVariantIndex;
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_linkage::linkage::Linkage;
use husky_task::{
    dev_ascension::IsRuntimeStorage,
    helpers::{TaskDevAscension, TaskKiControlFlow, TaskLinkageImpl, TaskValueResult},
};
use husky_task::{
    helpers::{DevRuntimeStorage, TaskDevLinkTime, TaskDevPedestal},
    IsTask,
};
use husky_task_interface::{
    ki_repr::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface},
    IsDevRuntime, IsLinkageImpl, LinkageImplKiControlFlow, TaskIngredientIndex, TaskJarIndex,
};
use husky_vfs::{error::VfsResult, linktime_target_path::LinktimeTargetPath};

use std::{convert::Infallible, path::Path};

/// Dropping libraries or linkage_impls before runtime storage will lead to segmentation fault
///
/// so it's necessary to pub `storage` field before `comptime`
pub struct DevRuntime<Task: IsTask> {
    task: Task,
    config: DevRuntimeConfig<Task>,
    storage: DevRuntimeStorage<Task>,
    comptime: DevComptime<Task>,
}

impl<Task: IsTask> DevRuntime<Task> {
    pub fn new(
        task: Task,
        target_crate: impl AsRef<Path>,
        config: Option<DevRuntimeConfig<Task>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            task,
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

impl<Task: IsTask> Default for DevRuntime<Task>
where
    Task: Default,
    TaskDevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            task: Default::default(),
            comptime: Default::default(),
            storage: Default::default(),
            config: Default::default(),
        }
    }
}

impl<Task: IsTask> IsDevRuntime<TaskLinkageImpl<Task>> for DevRuntime<Task> {
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_ingredient_at_pedestal_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: TaskDevPedestal<Task>,
        f: impl FnOnce() -> TaskValueResult<Task>,
    ) -> TaskKiControlFlow<Task> {
        self.storage.get_or_try_init_val_value(
            self.comptime.ingredient_val(jar_index, ingredient_index),
            base_point,
            f,
            self.db(),
        )
    }

    fn eval_ingredient_at_pedestal(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: <TaskLinkageImpl<Task> as IsLinkageImpl>::Pedestal,
    ) -> TaskKiControlFlow<Task> {
        self.eval_ki_repr_at_pedestal(
            self.comptime
                .ingredient_ki_repr(jar_index, ingredient_index),
            pedestal,
        )
    }

    fn eval_ki_repr_interface_at_pedestal(
        &self,
        ki_repr_interface: KiReprInterface,
        pedestal: <TaskLinkageImpl<Task> as IsLinkageImpl>::Pedestal,
    ) -> LinkageImplKiControlFlow<TaskLinkageImpl<Task>> {
        self.eval_ki_repr_at_pedestal(ki_repr_interface.into(), pedestal)
    }

    fn eval_ki_domain_repr_interface_at_pedestal(
        &self,
        ki_domain_repr: KiDomainReprInterface,
        pedestal: <TaskLinkageImpl<Task> as IsLinkageImpl>::Pedestal,
    ) -> husky_task_interface::ki_control_flow::KiControlFlow<
        (),
        Infallible,
        <TaskLinkageImpl<Task> as IsLinkageImpl>::Exception,
    > {
        self.eval_ki_domain_repr_at_pedestal(ki_domain_repr.into(), pedestal)
    }

    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        pedestal: <TaskLinkageImpl<Task> as IsLinkageImpl>::Pedestal,
        f: impl FnOnce(KiDomainReprInterface) -> LinkageImplKiControlFlow<TaskLinkageImpl<Task>>,
    ) -> LinkageImplKiControlFlow<TaskLinkageImpl<Task>> {
        let db = self.db();
        let ki_repr: KiRepr = unsafe { std::mem::transmute(ki_repr) };
        let ki_domain_repr: KiDomainReprInterface =
            unsafe { std::mem::transmute(ki_repr.ki_domain_repr(db)) };
        self.storage
            .get_or_try_init_val_value(ki_repr.val(db), pedestal, || f(ki_domain_repr), db)
    }

    fn eval_memo_field_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: <TaskLinkageImpl<Task> as IsLinkageImpl>::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<TaskLinkageImpl<Task>>,
    ) -> LinkageImplKiControlFlow<TaskLinkageImpl<Task>> {
        self.storage
            .get_or_try_init_memo_field_value(jar_index, ingredient_index, pedestal, slf, f)
    }

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> <TaskLinkageImpl<Task> as IsLinkageImpl>::Value {
        use husky_value_interface::IsValue;

        let db = self.db();
        let val_runtime_constant: KiRuntimeConstant =
            unsafe { std::mem::transmute(val_runtime_constant) };
        match val_runtime_constant.data(db) {
            KiRuntimeConstantData::TypeVariantPath(path) => {
                let presenter = self
                    .comptime
                    .linkage_impl(Linkage::new_enum_u8_presenter(path.parent_ty_path(db), db))
                    .enum_u8_value_presenter();
                match path.index(db) {
                    TypeVariantIndex::U8(raw) => {
                        <TaskLinkageImpl<Task> as IsLinkageImpl>::Value::from_enum_u8(
                            raw, presenter,
                        )
                    }
                }
            }
        }
    }
}
