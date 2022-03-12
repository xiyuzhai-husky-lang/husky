use std::cmp::min;

use common::*;
use semantics::Config;

use crate::*;
use vm::{AnyValueDyn, VMValue};

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn update_config(&mut self, config: &Arc<Config>) {
        if config != &self.config {
            // self.config = config.clone();
            todo!()
        }
    }

    pub(super) fn update_validation_report(&mut self) {
        let val_loader = self.dataset.val_loader();
        let len = min(val_loader.len(), 100);
        self.validation_report.predictions = (0..len)
            .map(|i: usize| self.offline_eval(self.main, i))
            .collect()
    }

    pub(super) fn update_main(&mut self, package: &Package) {
        self.main = self.parse_body(&package.main.stmts)
    }
}

fn test() {
    let a: Vec<i32> = (1i32..2i32).map(|i| i + 1).collect();
}

#[test]
fn test_arc_compare() {
    let a = Arc::new(0);
    let b = Arc::new(0);
    should_eq!(a, b);
    should_eq!(&a, &b);
}
