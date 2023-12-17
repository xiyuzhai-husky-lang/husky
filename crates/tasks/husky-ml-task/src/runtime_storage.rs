use crate::*;
use dashmap::DashMap;

use husky_standard_value::Value;
use husky_task::dev_ascension::IsRuntimeStorage;
use husky_val::{version_stamp::ValVersionStamp, Val};
use std::sync::{Arc, Mutex};

pub type ValueResult = Result<Value, ()>;

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    gn_values:
        DashMap<MlDevRuntimeGnStorageKey, Arc<Mutex<Option<(ValVersionStamp, ValueResult)>>>>,
    val_item_values:
        DashMap<MlDevRuntimeValItemStorageKey, Arc<Mutex<Option<(ValVersionStamp, ValueResult)>>>>,
    memoized_field_values:
        DashMap<MlDevRuntimeValItemStorageKey, Arc<Mutex<Option<(ValVersionStamp, ValueResult)>>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeGnStorageKey {
    val: Val,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeValItemStorageKey {
    val: Val,
    input_id: InputId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeMemoizedFieldStorageKey {
    val: Val,
}

impl IsRuntimeStorage<LinkageImpl> for MlDevRuntimeStorage {
    fn get_or_try_init_gn_value(
        &self,
        val: Val,
        f: impl FnOnce() -> ValueResult,
        db: &salsa::Db,
    ) -> ValueResult {
        todo!()
    }

    fn get_or_try_init_val_item_value(
        &self,
        val: Val,
        input_id: InputId,
        f: impl FnOnce() -> ValueResult,
        db: &::salsa::Db,
    ) -> ValueResult {
        let key = MlDevRuntimeValItemStorageKey { val, input_id };
        fn share(result: &ValueResult) -> ValueResult {
            match result {
                Ok(ref value) => Ok(value.share()),
                Err(_) => todo!(),
            }
        }

        let mu = self.val_item_values.entry(key).or_default().clone();
        let mut opt_stored_value = mu.lock().expect("todo");
        let new_version_stamp = key.val.version_stamp(db);
        match *opt_stored_value {
            Some((old_version_stamp, ref result)) if old_version_stamp == new_version_stamp => {
                return share(result)
            }
            _ => *opt_stored_value = Some((new_version_stamp, f())),
        };
        share(&opt_stored_value.as_ref().expect("should be some").1)
    }

    fn get_or_try_init_memoized_field_value(
        &self,
        f: impl FnOnce() -> ValueResult,
        db: &salsa::Db,
    ) -> ValueResult {
        todo!()
    }
}
