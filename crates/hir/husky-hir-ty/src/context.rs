use crate::{HirTemplateArgument, HirType};
use either::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    major_item::{form::PreludeMajorFormPath, MajorItemPath},
    ItemPath,
};
use husky_eth_term::{context::EthTermContextItd, instantiation::EthInstantiation};
use husky_fly_term::instantiation::FlyInstantiation;
use husky_sem_var_deps::{item_sem_var_deps, var_deps::SemVarDep};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTypeContext {
    /// overrides of type var, compterm var, lifetime var
    comptime_var_overrides: OrderedSmallVecPairMap<ItemPath, Option<HirComptimeVarOverride>, 4>,
    // todo: trait constraints
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirComptimeVarOverride {
    /// type var
    Type(HirType),
    // todo: compterm var, lifetime var
}

impl HirTypeContext {
    /// at this stage, we have var deps available to use, thus we're able to include only those necessary
    pub(crate) fn from_eth(eth_instantiation: &EthInstantiation, db: &::salsa::Db) -> Self {
        let path = eth_instantiation.path();
        let context_itd = eth_instantiation.context_itd();
        let mut comptime_var_overrides: OrderedSmallVecPairMap<
            ItemPath,
            Option<HirComptimeVarOverride>,
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
            Option<HirComptimeVarOverride>,
            4,
        > = Default::default();
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
                                MajorFormKind::TypeAlias => (),
                                MajorFormKind::TypeVar => {
                                    // ad hoc
                                    let ovrd = match dep_major_form_path.refine(db) {
                                        Left(dep_prelude_major_form_path) => {
                                            match dep_prelude_major_form_path {
                                                PreludeMajorFormPath::TaskType => {
                                                    context_itd.task_ty(db).map(|term| {
                                                      HirType::from_eth(term, db)
                                                            .expect("this should be always some because task_ty is always a valid type")
                                                            .into()
                                                    })
                                                }
                                            }
                                        }
                                        Right(_) => todo!(),
                                    };
                                    comptime_var_overrides.insert((dep_item_path, ovrd));
                                }
                                MajorFormKind::Val => todo!(),
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
            comptime_var_overrides,
        }
    }
}

impl HirTypeContext {
    pub fn comptime_var_overrides(
        &self,
    ) -> &OrderedSmallVecPairMap<ItemPath, Option<HirComptimeVarOverride>, 4> {
        &self.comptime_var_overrides
    }
}
