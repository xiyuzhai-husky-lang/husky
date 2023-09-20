#![feature(try_trait_v2)]

mod config;
mod db;
mod evaluator;
mod hot_reload;
mod impl_necessary;
mod impl_train;

pub use self::config::*;

use self::db::*;
use husky_check_utils::*;
use husky_compiler::CompilerInstance;
use husky_dev_comptime::DevComptime;
use husky_eval::*;
use husky_eval::{Runtime, Session};
use husky_print_utils::*;
use husky_task::{helpers::DevRuntimeStorage, IsTask};
use husky_vfs::{CratePath, DiffPathBuf};
use indexmap::IndexMap;
use relative_path::RelativePathBuf;
use std::{path::Path, sync::Arc};
use sync_utils::ASafeRwLock;

pub struct DevRuntime<Task: IsTask> {
    task: Task,
    comptime: DevComptime<Task>,
    storage: DevRuntimeStorage<Task>,
    config: DevRuntimeConfig<Task>,
}

impl<Task: IsTask> DevRuntime<Task> {
    pub fn new(task: Task, target_crate: &Path, config: Option<DevRuntimeConfig<Task>>) -> Self {
        Self {
            task,
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate),
        }
    }
}

// impl DevRuntime {
//     pub fn new(config: RuntimeConfig) -> DevRuntime {
//         todo!()
//         // let mut runtime = Self {
//         //     storage: Default::default(),
//         //     variant: HuskyRuntimeVariant::None,
//         //     feature_interner: Default::default(),
//         //     // comptime
//         //     file_interner: Default::default(),
//         //     word_interner: Default::default(),
//         //     live_docs: Default::default(),
//         //     linkage_table: LinkageTable::new(config.comptime.linkage_table.clone()),
//         //     item_route_store: Default::default(),
//         //     item_route_interner: Default::default(),
//         //     // config
//         //     config,
//         // };
//         // runtime.init();
//         // runtime
//     }

//     fn init(&mut self) {
//         self.hot_reload();
//         let all_main_files = self.all_target_entrances();
//         should_eq!(all_main_files.len(), 1, "config = {:?}", self.config);
//         let all_diagnostics = self.all_diagnostics();
//         if all_diagnostics.len() > 0 {
//             p!(all_diagnostics);
//             panic!("diagnostic errors")
//         }
//         let package = match self.package(self.opt_target_entrance().unwrap()) {
//             Ok(package) => package,
//             Err(error) => {
//                 self.print_diagnostics();
//                 p!(error);
//                 panic!()
//             }
//         };
//         self.variant = HuskyRuntimeVariant::Learning {
//             session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
//         }
//     }
// }
