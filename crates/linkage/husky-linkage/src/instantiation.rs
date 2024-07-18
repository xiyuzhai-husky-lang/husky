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
    context: LinTypeContext,
    symbol_resolutions: SmallVecPairMap<HirTemplateVariable, LinTermSymbolResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for LinInstantiation {
    type Target = [(HirTemplateVariable, LinTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_resolutions
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinTermSymbolResolution {
    Explicit(LinTemplateArgument),
    SelfLifetime,
    SelfQual(LinQual),
}

impl LinInstantiation {
    pub fn new_empty(is_associated: bool) -> Self {
        LinInstantiation {
            // todo: is this correct?
            context: LinTypeContext::new_empty(),
            symbol_resolutions: Default::default(),
            separator: is_associated.then_some(0),
        }
    }

    #[track_caller]
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> LinInstantiation {
        let symbol_resolutions =
            SmallVecMap::from_iter(hir_instantiation.symbol_map().iter().filter_map(
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
                        LinTermSymbolResolution::from_hir(resolution, lin_instantiation, db),
                    ))
                },
            ));
        let separator = hir_instantiation.separator();
        LinInstantiation {
            context: LinTypeContext::from_hir(hir_instantiation.context(), lin_instantiation, db),
            symbol_resolutions,
            separator,
        }
    }

    #[track_caller]
    pub(crate) fn resolve(&self, symbol: HirTemplateVariable) -> LinTermSymbolResolution {
        self.symbol_resolutions[symbol].1
    }

    pub fn places(&self) -> SmallVec<[(HirTemplateVariable, LinTermSymbolResolution); 2]> {
        self.symbol_resolutions
            .iter()
            .filter_map(|&(symbol, resolution)| match resolution {
                LinTermSymbolResolution::Explicit(LinTemplateArgument::Qual(_))
                | LinTermSymbolResolution::SelfQual(_) => Some((symbol, resolution)),
                LinTermSymbolResolution::Explicit(_) | LinTermSymbolResolution::SelfLifetime => {
                    None
                }
            })
            .collect()
    }

    pub fn symbol_resolutions(&self) -> &[(HirTemplateVariable, LinTermSymbolResolution)] {
        self.symbol_resolutions.as_ref()
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
        Self::from_jav_aux(
            jav_instantiation,
            LinInstantiation {
                context: LinTypeContext::from_jav(jav_instantiation.context(), db),
                symbol_resolutions: Default::default(),
                separator: jav_instantiation.separator,
            },
            &mut lin_instantiations,
            db,
        );
        lin_instantiations
    }

    fn from_jav_aux(
        jav_instantiation: &JavInstantiation,
        prefix: LinInstantiation,
        lin_instantiations: &mut SmallVec<[Self; 4]>,
        db: &::salsa::Db,
    ) {
        if prefix.len() == jav_instantiation.len() {
            lin_instantiations.push(prefix);
            return;
        }
        let (symbol, javelin_resolution) =
            jav_instantiation.symbol_resolutions.data()[prefix.len()];
        let linkage_resolutions =
            LinTermSymbolResolution::from_jav(javelin_resolution, &prefix, db);
        for linkage_resolution in linkage_resolutions {
            let mut prefix = prefix.clone();
            unsafe {
                prefix
                    .symbol_resolutions
                    .insert_new_unchecked((symbol, linkage_resolution))
            };
            Self::from_jav_aux(jav_instantiation, prefix, lin_instantiations, db)
        }
    }
}

impl LinTermSymbolResolution {
    fn from_jav(
        javelin_resolution: JavTermSymbolResolution,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        match javelin_resolution {
            JavTermSymbolResolution::Explicit(arg) => match arg {
                JavTemplateArgument::Vacant => todo!(),
                JavTemplateArgument::Type(javelin_ty) => {
                    smallvec![LinTermSymbolResolution::Explicit(
                        LinTemplateArgument::Type(LinType::from_jav(
                            javelin_ty,
                            lin_instantiation,
                            db
                        ))
                    )]
                }
                JavTemplateArgument::Constant(constant) => {
                    smallvec![LinTermSymbolResolution::Explicit(
                        LinTemplateArgument::Constant(LinConstant(constant))
                    )]
                }
                JavTemplateArgument::Lifetime => todo!(),
                JavTemplateArgument::Place => todo!(),
            },
            JavTermSymbolResolution::SelfLifetime => {
                smallvec![LinTermSymbolResolution::SelfLifetime]
            }
            JavTermSymbolResolution::SelfPlace => {
                smallvec![
                    LinTermSymbolResolution::SelfQual(LinQual::Ref),
                    LinTermSymbolResolution::SelfQual(LinQual::RefMut),
                ]
            }
        }
    }

    fn from_hir(
        resolution: HirTermSymbolicVariableResolution,
        lin_instantiation: &LinInstantiation,
        db: &salsa::Db,
    ) -> LinTermSymbolResolution {
        match resolution {
            HirTermSymbolicVariableResolution::Explicit(arg) => LinTermSymbolResolution::Explicit(
                LinTemplateArgument::from_hir(arg, Some(lin_instantiation), db),
            ),
            HirTermSymbolicVariableResolution::SelfLifetime => {
                LinTermSymbolResolution::SelfLifetime
            }
            HirTermSymbolicVariableResolution::SelfContractedQuary(contracted_quary) => {
                LinTermSymbolResolution::SelfQual(LinQual::from_hir(contracted_quary))
            }
        }
    }
}

pub trait LinkageInstantiate {
    type Output;

    fn linkage_instantiate(
        self,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self::Output;
}
