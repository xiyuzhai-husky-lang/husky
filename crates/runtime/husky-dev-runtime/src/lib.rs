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
    helpers::{DevRuntimeStorage, TaskDevBasePoint, TaskDevLinkTime},
    IsTask,
};
use husky_task_prelude::{
    IsDevRuntime, IsDevRuntimeDyn, LinkageImplValueResult, TaskIngredientIndex, TaskJarIndex,
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

    fn eval_val_item(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: TaskDevBasePoint<Task>,
        f: impl FnOnce() -> TaskValueResult<Task>,
    ) -> <TaskLinkageImpl<Task> as husky_task_prelude::IsLinkageImpl>::Value {
        let target_path = self.linktime_target_path().unwrap();
        let db = self.db();
        // todo: use comptime cached vals instead
        let val: Val = self.comptime.ingredient_val(jar_index, ingredient_index);
        &self
            .storage
            .get_or_try_init_val_item_value(val, base_point, f, self.db());
        todo!()
    }
}
