use crate::HirTemplateArgument;
use husky_entity_path::path::ItemPath;
use husky_eth_term::{context::EthTermContextItd, instantiation::EthInstantiation};
use husky_fly_term::instantiation::FlyInstantiation;
use husky_sem_var_deps::item_sem_var_deps;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTypeContext {
    /// overrides of type var, compterm var, lifetime var
    comptime_var_overrides: OrderedSmallVecPairMap<ItemPath, Option<HirTemplateArgument>, 4>,
    // todo: trait constraints
}

impl HirTypeContext {
    /// at this stage, we have var deps available to use, thus we're able to include only those necessary
    pub(crate) fn from_eth(eth_instantiation: &EthInstantiation, db: &::salsa::Db) -> Self {
        let path = eth_instantiation.path();
        let context_itd = eth_instantiation.context_itd();
        let mut comptime_var_overrides: OrderedSmallVecPairMap<
            ItemPath,
            Option<HirTemplateArgument>,
            4,
        > = Default::default();
        for _ in item_sem_var_deps(path, db) {
            todo!()
        }
        Self {
            comptime_var_overrides,
        }
    }

    /// at this stage, we have var deps available to use, thus we're able to include only those necessary
    pub(crate) fn from_fly(fly_instantiation: &FlyInstantiation, db: &::salsa::Db) -> Self {
        let path = fly_instantiation.path();
        let context_itd = fly_instantiation.context_itd();
        let mut comptime_var_overrides: OrderedSmallVecPairMap<
            ItemPath,
            Option<HirTemplateArgument>,
            4,
        > = Default::default();
        for _ in item_sem_var_deps(path, db) {
            todo!()
        }
        Self {
            comptime_var_overrides,
        }
    }
}
