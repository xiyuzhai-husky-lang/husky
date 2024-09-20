use husky_entity_path::path::{ty_variant::TypeVariantPath, ItemPath};
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolicVariableResolution},
    HirTemplateVariable, HirTemplateVariableClass,
};
use husky_javelin::{
    instantiation::{JavInstantiation, JavTermSymbolResolution},
    template_argument::JavTemplateArgument,
};

use smallvec::*;
use vec_like::{SmallVecMap, SmallVecPairMap};

use crate::{
    context::LinTypeContext,
    template_argument::{constant::LinConstant, qual::LinQual, ty::LinType, LinTemplateArgument},
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LinInstantiation {
    path: ItemPath,
    context: LinTypeContext,
    variable_resolutions: SmallVecPairMap<HirTemplateVariable, LinTermVariableResolution, 4>,
    separator: Option<u8>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinTermVariableResolution {
    Explicit(LinTemplateArgument),
    SelfLifetime,
    SelfQual(LinQual),
}

impl LinInstantiation {
    pub fn new_empty(path: impl Into<ItemPath>, is_associated: bool) -> Self {
        LinInstantiation {
            path: path.into(),
            // todo: is this correct?
            context: LinTypeContext::new_empty(),
            variable_resolutions: Default::default(),
            separator: is_associated.then_some(0),
        }
    }

    /// this is quite casual. We don't have any complications like nondeterminism for comptime vars,
    /// as compared with symbol resolutions, by design.
    pub(crate) fn new_empty_for_comptime_var_overrides(path: ItemPath) -> Self {
        Self::new_empty(path, false)
    }

    #[track_caller]
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> LinInstantiation {
        let variable_resolutions =
            SmallVecMap::from_iter(hir_instantiation.variable_map().iter().filter_map(
                |&(symbol, resolution)| {
                    match symbol {
                        HirTemplateVariable::Compterm(symbol)
                            if symbol.index(db).class() == HirTemplateVariableClass::Poly =>
                        {
                            return None
                        }
                        _ => (),
                    }
                    Some((
                        symbol,
                        LinTermVariableResolution::from_hir(resolution, lin_instantiation, db),
                    ))
                },
            ));
        let separator = hir_instantiation.separator();
        LinInstantiation {
            path: hir_instantiation.path(),
            context: LinTypeContext::from_hir(hir_instantiation.context(), lin_instantiation, db),
            variable_resolutions,
            separator,
        }
    }

    pub(crate) fn new_ty_variant(&self, path: TypeVariantPath) -> Self {
        Self {
            path: path.into(),
            context: self.context.clone(),
            variable_resolutions: self.variable_resolutions.clone(),
            separator: self.separator,
        }
    }
}

impl LinInstantiation {
    pub fn path(&self) -> ItemPath {
        self.path
    }

    pub fn is_empty(&self) -> bool {
        self.context.comptime_var_overrides().is_empty() && self.variable_resolutions.is_empty()
    }

    #[track_caller]
    pub(crate) fn resolve(&self, variable: HirTemplateVariable) -> LinTermVariableResolution {
        self.variable_resolutions[variable].1
    }

    pub fn places(&self) -> SmallVec<[(HirTemplateVariable, LinTermVariableResolution); 2]> {
        self.variable_resolutions
            .iter()
            .filter_map(|&(variable, resolution)| match resolution {
                LinTermVariableResolution::Explicit(LinTemplateArgument::Qual(_))
                | LinTermVariableResolution::SelfQual(_) => Some((variable, resolution)),
                LinTermVariableResolution::Explicit(_)
                | LinTermVariableResolution::SelfLifetime => None,
            })
            .collect()
    }

    pub fn variable_resolutions(&self) -> &[(HirTemplateVariable, LinTermVariableResolution)] {
        self.variable_resolutions.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn context(&self) -> &LinTypeContext {
        &self.context
    }
}

impl LinInstantiation {
    /// a nondeterminstic map basically
    pub(crate) fn from_jav(
        jav_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        let mut lin_instantiations = smallvec![];
        let prefix_lin_instantiation = LinInstantiation {
            path: jav_instantiation.path(),
            context: LinTypeContext::new_empty(),
            variable_resolutions: Default::default(),
            separator: jav_instantiation.separator,
        };
        let prefix = LinInstantiation {
            path: jav_instantiation.path(),
            context: LinTypeContext::from_jav(
                jav_instantiation.context(),
                &prefix_lin_instantiation,
                db,
            ),
            variable_resolutions: Default::default(),
            separator: jav_instantiation.separator,
        };
        Self::from_jav_aux(jav_instantiation, prefix, &mut lin_instantiations, db);
        lin_instantiations
    }

    fn from_jav_aux(
        jav_instantiation: &JavInstantiation,
        prefix: LinInstantiation,
        lin_instantiations: &mut SmallVec<[Self; 4]>,
        db: &::salsa::Db,
    ) {
        if prefix.variable_resolutions.len() == jav_instantiation.variable_resolutions.len() {
            lin_instantiations.push(prefix);
            return;
        }
        let (symbol, javelin_resolution) =
            jav_instantiation.variable_resolutions.data()[prefix.variable_resolutions.len()];
        let linket_resolutions =
            LinTermVariableResolution::from_jav(javelin_resolution, &prefix, db);
        for linket_resolution in linket_resolutions {
            let mut prefix = prefix.clone();
            unsafe {
                prefix
                    .variable_resolutions
                    .insert_new_unchecked((symbol, linket_resolution))
            };
            Self::from_jav_aux(jav_instantiation, prefix, lin_instantiations, db)
        }
    }
}

impl LinTermVariableResolution {
    fn from_jav(
        javelin_resolution: JavTermSymbolResolution,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        match javelin_resolution {
            JavTermSymbolResolution::Explicit(arg) => match arg {
                JavTemplateArgument::Vacant => todo!(),
                JavTemplateArgument::Type(javelin_ty) => {
                    smallvec![LinTermVariableResolution::Explicit(
                        LinTemplateArgument::Type(LinType::from_jav(
                            javelin_ty,
                            lin_instantiation,
                            db
                        ))
                    )]
                }
                JavTemplateArgument::Constant(constant) => {
                    smallvec![LinTermVariableResolution::Explicit(
                        LinTemplateArgument::Constant(LinConstant(constant))
                    )]
                }
                JavTemplateArgument::Lifetime => todo!(),
                JavTemplateArgument::Place => todo!(),
            },
            JavTermSymbolResolution::SelfLifetime => {
                smallvec![LinTermVariableResolution::SelfLifetime]
            }
            JavTermSymbolResolution::SelfPlace => {
                smallvec![
                    LinTermVariableResolution::SelfQual(LinQual::Ref),
                    LinTermVariableResolution::SelfQual(LinQual::RefMut),
                ]
            }
        }
    }

    fn from_hir(
        resolution: HirTermSymbolicVariableResolution,
        instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> LinTermVariableResolution {
        match resolution {
            HirTermSymbolicVariableResolution::Explicit(arg) => {
                LinTermVariableResolution::Explicit(LinTemplateArgument::from_hir(
                    arg,
                    instantiation,
                    db,
                ))
            }
            HirTermSymbolicVariableResolution::SelfLifetime => {
                LinTermVariableResolution::SelfLifetime
            }
            HirTermSymbolicVariableResolution::SelfContractedQuary(contracted_quary) => {
                LinTermVariableResolution::SelfQual(LinQual::from_hir(contracted_quary))
            }
        }
    }
}

pub trait LinInstantiate {
    type Output;

    fn lin_instantiate(
        self,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self::Output;
}
