use crate::DevInput;
use dashmap::DashMap;

use husky_regular_value::RegularValue;
use husky_val::{version_stamp::ValVersionStamp, Val};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    map: DashMap<
        MlDevRuntimeStorageKey,
        Arc<Mutex<Option<(ValVersionStamp, VMResult<RegularValue>)>>>,
    >,
}

// ad hoc
unsafe impl Send for MlDevRuntimeStorage {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MlDevRuntimeStorageKey {
    val: Val,
    input: Option<DevInput>,
}

type VMResult<T> = Result<T, ()>;

impl MlDevRuntimeStorage {
    pub fn get_or_try_init<E>(
        &self,
        key: MlDevRuntimeStorageKey,
        f: impl FnOnce() -> VMResult<RegularValue>,
        db: &::salsa::Db,
    ) -> VMResult<RegularValue> {
        fn share(result: &VMResult<RegularValue>) -> VMResult<RegularValue> {
            match result {
                Ok(ref value) => Ok(value.share()),
                Err(_) => todo!(),
            }
        }

        let mu = self.map.entry(key).or_default().clone();
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
}
