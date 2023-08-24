#![feature(try_trait_v2)]

mod comptime;
mod config;
mod db;
mod hot_reload;
mod impl_necessary;
mod impl_train;
mod variant;

pub use self::config::*;

use self::db::*;
use husky_check_utils::*;
use husky_compiler::CompilerInstance;
use husky_comptime::Comptime;
use husky_eval::*;
use husky_eval::{Runtime, Session};
use husky_print_utils::*;
use husky_task::{DevRuntimeStorage, IsAscension, IsTask};
use husky_vfs::CratePath;
use indexmap::IndexMap;
use relative_path::RelativePathBuf;
use std::sync::Arc;
use sync_utils::ASafeRwLock;
use variant::*;

pub struct DevRuntime<'a, Task: IsTask> {
    storage: DevRuntimeStorage<Task>,
    comptime: &'a Comptime,
    crate_path: CratePath,
    task: Task,
    config: DevRuntimeConfig<Task>,
}

impl<'a, Task: IsTask> DevRuntime<'a, Task> {
    fn new(
        comptime: &'a Comptime,
        crate_path: CratePath,
        task: Task,
        config: Option<DevRuntimeConfig<Task>>,
    ) -> Self {
        Self {
            storage: Default::default(),
            comptime,
            crate_path,
            task,
            config: config.unwrap_or_default(),
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
