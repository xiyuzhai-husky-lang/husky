use crate::DevInput;
use dashmap::{
    mapref::entry::{Entry, VacantEntry},
    DashMap,
};
use either::*;
use husky_regular_value::RegularValue;
use husky_val::{deps::ValDeps, Val, ValDb};
use husky_vm::VMResult;
use std::{
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    map: DashMap<MlDevRuntimeStorageKey, Arc<Mutex<Option<(ValDeps, VMResult<RegularValue>)>>>>,
}

// ad hoc
unsafe impl Send for MlDevRuntimeStorage {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeStorageKey {
    val: Val,
    input: Option<DevInput>,
}

impl MlDevRuntimeStorage {
    pub fn get_or_try_init<E>(
        &self,
        key: MlDevRuntimeStorageKey,
        f: impl FnOnce() -> VMResult<RegularValue>,
        db: &dyn ValDb,
    ) -> VMResult<RegularValue> {
        fn share(result: &VMResult<RegularValue>) -> VMResult<RegularValue> {
            match result {
                Ok(ref value) => Ok(value.share()),
                Err(_) => todo!(),
            }
        }

        let mu = self.map.entry(key).or_default().clone();
        let mut opt_stored_value = mu.lock().expect("todo");
        let new_deps = key.val.deps(db);
        match *opt_stored_value {
            Some((old_deps, ref result)) if old_deps == new_deps => return share(result),
            _ => *opt_stored_value = Some((new_deps, f())),
        };
        share(&opt_stored_value.as_ref().expect("should be some").1)
    }
}
