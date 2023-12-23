#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::{
    ingredient::HasIngredientPaths, jar::package_path_from_jar_index,
};
use husky_task::{
    dev_ascension::{with_dev_eval_context, IsRuntimeStorage},
    helpers::{TaskDevAscension, TaskLinkageImpl, TaskValue, TaskValueResult},
};
use husky_task::{
    helpers::{DevRuntimeStorage, TaskDevLinkTime, TaskDevPedestal},
    IsTask,
};
use husky_task_prelude::{
    val_control_flow::ValControlFlow, val_repr::ValReprInterface, IsDevRuntime, IsDevRuntimeDyn,
    LinkageImplValControlFlow, TaskIngredientIndex, TaskJarIndex,
};
use husky_val::Val;
use husky_val_repr::repr::ValRepr;
use husky_vfs::{error::VfsResult, linktime_target_path::LinktimeTargetPath};

use std::path::Path;

pub struct DevRuntime<Task: IsTask> {
    task: Task,
    comptime: DevComptime<Task>,
    storage: DevRuntimeStorage<Task>,
    config: DevRuntimeConfig<Task>,
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

    fn eval_ingredient_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: TaskDevPedestal<Task>,
        f: impl FnOnce() -> TaskValueResult<Task>,
    ) -> <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Value {
        let target_path = self.linktime_target_path().unwrap();
        let db = self.db();
        let val: Val = self.comptime.ingredient_val(jar_index, ingredient_index);
        &self
            .storage
            .get_or_try_init_val_value(val, base_point, f, db);
        todo!()
    }

    fn eval_ingredient(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Pedestal,
    ) -> <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Value {
        let target_path = self.linktime_target_path().unwrap();
        let db = self.db();
        let val_repr: ValRepr = self
            .comptime
            .ingredient_val_repr(jar_index, ingredient_index);
        match self.eval_val_repr_at_pedestal(val_repr, pedestal) {
            ValControlFlow::Continue(value) => value,
            ValControlFlow::LoopContinue => todo!(),
            ValControlFlow::LoopBreak(_) => todo!(),
            ValControlFlow::Return(_) => todo!(),
            ValControlFlow::Undefined => todo!(),
            ValControlFlow::Err(_) => todo!(),
        }
    }

    fn eval_val_repr_interface_at_pedestal(
        &self,
        val_repr_interface: ValReprInterface,
        pedestal: <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Pedestal,
    ) -> LinkageImplValControlFlow<TaskLinkageImpl<Task>> {
        self.eval_val_repr_at_pedestal(unsafe { std::mem::transmute(val_repr_interface) }, pedestal)
    }

    fn eval_val_repr_with(
        &self,
        val_repr: ValReprInterface,
        pedestal: <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Pedestal,
        f: impl FnOnce() -> LinkageImplValControlFlow<TaskLinkageImpl<Task>>,
    ) -> LinkageImplValControlFlow<TaskLinkageImpl<Task>> {
        f();
        todo!()
    }
}
