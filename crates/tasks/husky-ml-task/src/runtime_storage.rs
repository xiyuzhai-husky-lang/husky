use crate::*;
use dashmap::DashMap;
use husky_linkage_impl::standard::ValControlFlow;

use husky_task::dev_ascension::IsRuntimeStorage;
use husky_task_interface::TaskJarIndex;
use husky_task_interface::{LinkageImplValControlFlow, TaskIngredientIndex};
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
    memoized_field_values:
        DashMap<MlDevRuntimeMemoizedFieldStorageKey, Arc<Mutex<Option<ValControlFlow>>>>,
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
    jar_index: TaskJarIndex,
    ingredient_index: TaskIngredientIndex,
    pedestal: MlPedestal,
    slf: AnyPointer,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct AnyPointer(*const std::ffi::c_void);

unsafe impl Send for AnyPointer {}

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
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: MlPedestal,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> ValControlFlow {
        // todo: maybe add version stamp?
        let key = MlDevRuntimeMemoizedFieldStorageKey {
            jar_index,
            ingredient_index,
            pedestal,
            slf: AnyPointer(slf as _),
        };
        let mu = self.memoized_field_values.entry(key).or_default().clone();
        let mut opt_stored_val_control_flow_store_guard = mu.lock().expect("todo");
        unsafe {
            match *opt_stored_val_control_flow_store_guard {
                Some(ref val_control_flow) => val_control_flow.share_unchecked(),
                None => {
                    *opt_stored_val_control_flow_store_guard = Some(f(slf));
                    opt_stored_val_control_flow_store_guard
                        .as_ref()
                        .expect("should be some")
                        .share_unchecked()
                }
            }
        }
    }

    fn debug_drop(self) {
        
        println!("{}", self.val_item_values.len());
        self.val_item_values.iter().for_each(|_| ());
        // forget(self.val_item_values);
        // forget(self.memoized_field_values);
        // todo!();
        // for (key, mu) in self.val_item_values {
        //     todo!();
        //     let lock = &mu.lock().unwrap();
        //     let (_, value) = lock.as_ref().unwrap();
        //     todo!();
        //     println!("{:?}", value);
        //     todo!();
        // }
        // todo!();
        // forget(self.memoized_field_values);
    }
}
