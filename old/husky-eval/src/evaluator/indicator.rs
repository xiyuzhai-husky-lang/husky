use husky_val::Val;

use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct EvalIndicator {
    set: HashSet<Val>,
}

impl EvalIndicator {
    pub fn set(&mut self, id: Val) {
        self.set.insert(id);
    }
}
