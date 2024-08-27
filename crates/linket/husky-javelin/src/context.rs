use crate::{
    instantiation::{JavInstantiation, JavTermSymbolResolution},
    template_argument::ty::JavType,
};
use either::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    major_item::{form::PreludeMajorFormPath, MajorItemPath},
    ItemPath,
};
use husky_hir_ty::context::{HirComptimeVarOverride, HirTypeContext};
use husky_sem_var_deps::{item_sem_var_deps, var_deps::SemVarDep};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavTypeContext {
    /// overrides of type var, compterm var, lifetime var
    comptime_var_overrides: OrderedSmallVecPairMap<ItemPath, JavComptimeVarOverride, 4>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JavComptimeVarOverride {
    /// type var
    Type(JavType),
    // todo: compterm var, lifetime var
}

impl JavTypeContext {
    pub(crate) fn from_hir(
        hir_context: &HirTypeContext,
        instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let comptime_var_overrides =
            hir_context
                .comptime_var_overrides()
                .map_collect2(|item_path, &arg| match arg {
                    Some(arg) => JavComptimeVarOverride::from_hir(arg, instantiation, db),
                    None => instantiation.context().comptime_var_overrides[item_path].1,
                });
        Self {
            comptime_var_overrides,
        }
    }

    pub(crate) fn new_amazon(path: ItemPath, db: &::salsa::Db) -> Self {
        #[cfg(test)]
        for dep in item_sem_var_deps(path, db) {
            match dep {
                SemVarDep::Item(dep_item_path) => match dep_item_path {
                    ItemPath::Submodule(_, _) => todo!(),
                    ItemPath::MajorItem(dep_major_item_path) => match dep_major_item_path {
                        MajorItemPath::Type(_) => todo!(),
                        MajorItemPath::Trait(_) => todo!(),
                        MajorItemPath::Form(dep_major_form_path) => {
                            match dep_major_form_path.kind(db) {
                                MajorFormKind::Ritchie(_) => todo!(),
                                MajorFormKind::TypeAlias => todo!(),
                                MajorFormKind::TypeVar => {
                                    unreachable!("amazon javelin shouldn't have any type var dep")
                                }
                                MajorFormKind::Val => (),
                                MajorFormKind::StaticMut => (),
                                MajorFormKind::StaticVar => (),
                                MajorFormKind::Compterm => todo!(),
                                MajorFormKind::Conceptual => todo!(),
                            }
                        }
                    },
                    ItemPath::AssocItem(_) => todo!(),
                    ItemPath::TypeVariant(_, _) => todo!(),
                    ItemPath::ImplBlock(_) => todo!(),
                    ItemPath::Attr(_, _) => todo!(),
                    ItemPath::Script(_, _) => todo!(),
                },
            }
        }
        Self {
            comptime_var_overrides: Default::default(),
        }
    }
}

impl JavTypeContext {
    pub fn is_empty(&self) -> bool {
        self.comptime_var_overrides.is_empty()
    }
}

impl JavComptimeVarOverride {
    fn from_hir(
        hir_ovrd: HirComptimeVarOverride,
        instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ovrd {
            HirComptimeVarOverride::Type(hir_ty) => {
                JavComptimeVarOverride::Type(JavType::from_hir(hir_ty, instantiation, db))
            }
        }
    }
}

/// # getters
impl JavTypeContext {
    pub fn comptime_var_overrides(
        &self,
    ) -> &OrderedSmallVecPairMap<ItemPath, JavComptimeVarOverride, 4> {
        &self.comptime_var_overrides
    }
}
