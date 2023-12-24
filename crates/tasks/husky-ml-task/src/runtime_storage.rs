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
        let mu = self.val_item_values.entry(key).or_default().clone();
        let mut opt_stored_val_control_flow_store_guard = mu.lock().expect("todo");
        let new_version_stamp = key.val.version_stamp(db);
        unsafe {
            match *opt_stored_val_control_flow_store_guard {
                Some((old_version_stamp, ref val_control_flow))
                    if old_version_stamp == new_version_stamp =>
                {
                    return val_control_flow.share_unchecked()
                }
                _ => *opt_stored_val_control_flow_store_guard = Some((new_version_stamp, f())),
            };
            opt_stored_val_control_flow_store_guard
                .as_ref()
                .expect("should be some")
                .1
                .share_unchecked()
        }
    }

    fn get_or_try_init_memoized_field_value(
        &self,
        f: impl FnOnce() -> ValControlFlow,
        db: &salsa::Db,
    ) -> ValControlFlow {
        todo!()
    }
}
