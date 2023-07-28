mod division;
mod tests;

pub use division::*;

use crate::*;
use husky_datasets_interface::{Dataset, __rust_code_gen__::__DATASET_VTABLE};
use husky_package_semantics::{Config, Package};
use husky_vm::{__VMResult, eval_fast};
use std::sync::Arc;

#[derive(Debug)]
pub struct Session {
    config: Arc<Config>,
    pub(crate) dev: Division,
    val: Division,
    test: Division,
}

impl Session {
    pub fn new(package: &Package, db: &dyn ValReprDb, vm_config: &VMConfig) -> __VMResult<Self> {
        let config = package.config.clone();
        let dataset: Dataset = eval_fast(
            db.upcast(),
            None,
            Some(&db.dataset_config_instruction_sheet(package.main_defn.file)),
            None,
            [].into_iter(),
            [].into_iter(),
            0,
            vm_config,
        )?
        .downcast_unbox(&__DATASET_VTABLE);
        Ok(Self {
            dev: Division::new(dataset.dev_loader()),
            val: Division::new(dataset.val_loader()),
            test: Division::new(dataset.test_loader()),
            config,
        })
    }

    pub fn dev(&self) -> &Division {
        &self.dev
    }

    pub fn val(&self) -> &Division {
        &self.val
    }
}
