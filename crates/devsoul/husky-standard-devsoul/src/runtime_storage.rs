use crate::*;
use dashmap::DashMap;
use husky_linkage_impl::standard::StandardLinkageImplKiControlFlow;

use husky_devsoul::devsoul::IsRuntimeStorage;
use husky_devsoul_interface::{HuskyIngredientIndex, HuskyJarIndex};
use husky_ki::{version_stamp::KiVersionStamp, Ki};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct StandardDevRuntimeStorage {
    ki_values: DashMap<
        StandardDevRuntimeValStorageKey,
        Arc<Mutex<Option<(KiVersionStamp, StandardLinkageImplKiControlFlow)>>>,
    >,
    memo_field_values: DashMap<
        StandardDevRuntimeMemoizedFieldStorageKey,
        Arc<Mutex<Option<StandardLinkageImplKiControlFlow>>>,
    >,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StandardDevRuntimeValStorageKey {
    ki: Ki,
    pedestal: StandardPedestal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StandardDevRuntimeMemoizedFieldStorageKey {
    jar_index: HuskyJarIndex,
    ingredient_index: HuskyIngredientIndex,
    slf: AnyPointer,
}

// todo: make a safer key than AnyPointer
// a pointer might not be unique
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct AnyPointer(*const std::ffi::c_void);

unsafe impl Send for AnyPointer {}

impl IsRuntimeStorage<LinkageImpl> for StandardDevRuntimeStorage {
    fn get_or_try_init_ki_value(
        &self,
        ki: Ki,
        f: impl FnOnce() -> StandardLinkageImplKiControlFlow,
        db: &::salsa::Db,
    ) -> StandardLinkageImplKiControlFlow {
        let pedestal = todo!();
        let key = StandardDevRuntimeValStorageKey { ki, pedestal };
        let mu = self.ki_values.entry(key).or_default().clone();
        let mut opt_stored_val_control_flow_store_guard = mu.lock().expect("todo");
        let new_version_stamp = key.ki.version_stamp(db);
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

    fn get_or_try_init_memo_field_value(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> StandardLinkageImplKiControlFlow,
    ) -> StandardLinkageImplKiControlFlow {
        // todo: maybe add version stamp?
        let key = StandardDevRuntimeMemoizedFieldStorageKey {
            jar_index,
            ingredient_index,
            slf: AnyPointer(slf as _),
        };
        let mu = self.memo_field_values.entry(key).or_default().clone();
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
}
