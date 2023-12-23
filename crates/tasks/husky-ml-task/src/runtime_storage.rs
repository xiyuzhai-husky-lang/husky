use crate::*;
use dashmap::DashMap;

use husky_linkage_impl::standard::ValControlFlow;
use husky_standard_value::Value;
use husky_task::dev_ascension::IsRuntimeStorage;
use husky_val::{version_stamp::ValVersionStamp, Val};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    gn_values:
        DashMap<MlDevRuntimeGnStorageKey, Arc<Mutex<Option<(ValVersionStamp, ValControlFlow)>>>>,
    val_item_values: DashMap<
        MlDevRuntimeValItemStorageKey,
        Arc<Mutex<Option<(ValVersionStamp, ValControlFlow)>>>,
    >,
    memoized_field_values: DashMap<
        MlDevRuntimeValItemStorageKey,
        Arc<Mutex<Option<(ValVersionStamp, ValControlFlow)>>>,
    >,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeGnStorageKey {
    val: Val,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeValItemStorageKey {
    val: Val,
    pedestal: MlPedestal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeMemoizedFieldStorageKey {
    val: Val,
}

impl IsRuntimeStorage<LinkageImpl> for MlDevRuntimeStorage {
    fn get_or_try_init_val_value(
        &self,
        val: Val,
        pedestal: MlPedestal,
        f: impl FnOnce() -> ValControlFlow,
        db: &::salsa::Db,
    ) -> ValControlFlow {
        let key = MlDevRuntimeValItemStorageKey { val, pedestal };
        fn share(result: &ValControlFlow) -> ValControlFlow {
            todo!()
            // match result {
            //     Ok(ref value) => Ok(value.share()),
            //     Err(_) => todo!(),
            // }
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
        f: impl FnOnce() -> ValControlFlow,
        db: &salsa::Db,
    ) -> ValControlFlow {
        todo!()
    }
}
