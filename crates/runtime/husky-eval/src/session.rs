mod division;
mod tests;

pub use division::*;
use husky_vfs::PackagePath;

use crate::*;
use husky_vm::{eval_fast, VMResult};
use std::sync::Arc;

#[derive(Debug)]
pub struct Session {
    // config: Arc<Config>,
    pub(crate) dev: Division,
    val: Division,
    test: Division,
}

impl Session {
    pub fn new(package: PackagePath, db: &dyn ValReprDb, vm_config: &VMConfig) -> VMResult<Self> {
        todo!()
        // let config = package.config.clone();
        // let dataset: Dataset = eval_fast(
        //     db.upcast(),
        //     None,
        //     Some(&db.dataset_config_instruction_sheet(package.main_defn.file)),
        //     None,
        //     [].into_iter(),
        //     [].into_iter(),
        //     0,
        //     vm_config,
        // )?
        // .downcast_unbox(&__DATASET_VTABLE);
        // Ok(Self {
        //     dev: Division::new(dataset.dev_loader()),
        //     val: Division::new(dataset.val_loader()),
        //     test: Division::new(dataset.test_loader()),
        //     config,
        // })
    }

    pub fn dev(&self) -> &Division {
        &self.dev
    }

    pub fn val(&self) -> &Division {
        &self.val
    }
}
