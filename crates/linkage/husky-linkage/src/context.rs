use crate::{
    template_argument::{ty::LinType, LinTemplateArgument},
    LinInstantiation,
};
use husky_entity_path::path::ItemPath;
use husky_hir_ty::context::{HirComptimeVarOverride, HirTypeContext};
use husky_javelin::context::{JavComptimeVarOverride, JavTypeContext};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LinTypeContext {
    /// overrides of type var, compterm var, lifetime var
    comptime_var_overrides: OrderedSmallVecPairMap<ItemPath, LinComptimeVarOverride, 4>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinComptimeVarOverride {
    /// type var
    Type(LinType),
    // todo: compterm var, lifetime var
}

impl LinTypeContext {
    pub(crate) fn from_hir(
        hir_context: &HirTypeContext,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let comptime_var_overrides =
            hir_context
                .comptime_var_overrides()
                .map_collect2(|item_path, &ovrd| match ovrd {
                    Some(ovrd) => LinComptimeVarOverride::from_hir(ovrd, instantiation, db),
                    None => instantiation.context().comptime_var_overrides[item_path].1,
                });
        Self {
            comptime_var_overrides,
        }
    }

    pub(crate) fn from_jav(jav_context: &JavTypeContext, db: &::salsa::Db) -> Self {
        let comptime_var_overrides = jav_context
            .comptime_var_overrides()
            .map_collect(|&ovrd| LinComptimeVarOverride::from_jav(ovrd, db));
        Self {
            comptime_var_overrides,
        }
    }

    pub(crate) fn new_empty() -> LinTypeContext {
        Self {
            comptime_var_overrides: Default::default(),
        }
    }
}

impl LinComptimeVarOverride {
    fn from_hir(
        hir_ovrd: HirComptimeVarOverride,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ovrd {
            HirComptimeVarOverride::Type(hir_ty) => {
                LinComptimeVarOverride::Type(LinType::from_hir(hir_ty, instantiation, db))
            }
        }
    }

    fn from_jav(jav_ovrd: JavComptimeVarOverride, db: &::salsa::Db) -> Self {
        match jav_ovrd {
            JavComptimeVarOverride::Type(jav_ty) => {
                LinComptimeVarOverride::Type(LinType::from_jav(jav_ty, todo!(), db))
            }
        }
    }
}

impl LinTypeContext {
    pub fn comptime_var_overrides(
        &self,
    ) -> &OrderedSmallVecPairMap<ItemPath, LinComptimeVarOverride, 4> {
        &self.comptime_var_overrides
    }
}
