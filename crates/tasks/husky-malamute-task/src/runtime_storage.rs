use crate::DevInput;
use dashmap::{
    mapref::entry::{Entry, VacantEntry},
    DashMap,
};
use either::*;
use husky_regular_value::RegularValue;
use husky_val::Val;
use std::panic::AssertUnwindSafe;

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    feature_maps: DashMap<MlDevRuntimeStorageKey, RegularValue>,
}

impl MlDevRuntimeStorage {
    pub fn get_value_or_guard(
        &self,
        key: MlDevRuntimeStorageKey,
    ) -> Either<RegularValue, MlDevRuntimeStorageGuard> {
        match self.feature_maps.entry(key) {
            Entry::Occupied(occupied_entry) => Left(occupied_entry.get().share()),
            Entry::Vacant(vacant_entry) => Right(MlDevRuntimeStorageGuard(vacant_entry)),
        }
    }
}

pub struct MlDevRuntimeStorageGuard<'a>(VacantEntry<'a, MlDevRuntimeStorageKey, RegularValue>);

#[derive(Debug, Default)]
pub struct FeatureMap {
    // data:Dashmap<>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MlDevRuntimeStorageKey {
    ModelInternal { val: Val },
    ConstantVal { val: Val },
    NonConstantVal { val: Val, input: DevInput },
}
