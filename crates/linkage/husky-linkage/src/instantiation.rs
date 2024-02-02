use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirTemplateSymbol, HirTemplateSymbolClass,
};
use husky_javelin::{
    instantiation::{JavInstantiation, JavTermSymbolResolution},
    template_argument::JavTemplateArgument,
};

use smallvec::*;
use vec_like::{SmallVecMap, SmallVecPairMap};

use crate::template_argument::{
    constant::LinConstant, place::LinPlace, ty::LinType, LinTemplateArgument,
};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LinkageInstantiation {
    symbol_resolutions: SmallVecPairMap<HirTemplateSymbol, LinTermSymbolResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for LinkageInstantiation {
    type Target = [(HirTemplateSymbol, LinTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_resolutions
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinTermSymbolResolution {
    Explicit(LinTemplateArgument),
    SelfLifetime,
    SelfPlace(LinPlace),
}

impl LinkageInstantiation {
    pub fn new_empty(is_associated: bool) -> Self {
        LinkageInstantiation {
            symbol_resolutions: Default::default(),
            separator: is_associated.then_some(0),
        }
    }

    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> LinkageInstantiation {
        let symbol_resolutions =
            SmallVecMap::from_iter(hir_instantiation.symbol_map().iter().filter_map(
                |&(symbol, resolution)| {
                    match symbol {
                        HirTemplateSymbol::Const(symbol)
                            if symbol.index(db).class() == HirTemplateSymbolClass::Runtime =>
                        {
                            return None
                        }
                        _ => (),
                    }
                    Some((
                        symbol,
                        LinTermSymbolResolution::from_hir(resolution, linkage_instantiation, db),
                    ))
                },
            ));
        let separator = hir_instantiation.separator();
        if let Some(separator) = separator {
            debug_assert!((separator as usize) <= symbol_resolutions.len());
        }
        LinkageInstantiation {
            symbol_resolutions,
            separator,
        }
    }

    #[track_caller]
    pub(crate) fn resolve(&self, symbol: HirTemplateSymbol) -> LinTermSymbolResolution {
        self.symbol_resolutions[symbol].1
    }

    pub fn places(&self) -> SmallVec<[(HirTemplateSymbol, LinTermSymbolResolution); 2]> {
        self.symbol_resolutions
            .iter()
            .filter_map(|&(symbol, resolution)| match resolution {
                LinTermSymbolResolution::Explicit(arg) => match arg {
                    LinTemplateArgument::Vacant => todo!(),
                    LinTemplateArgument::Type(_) => None,
                    LinTemplateArgument::Constant(_) => todo!(),
                    LinTemplateArgument::Lifetime => todo!(),
                    LinTemplateArgument::Place(_) => todo!(),
                },
                LinTermSymbolResolution::SelfLifetime => None,
                LinTermSymbolResolution::SelfPlace(_) => Some((symbol, resolution)),
            })
            .collect()
    }

    pub fn symbol_resolutions(&self) -> &[(HirTemplateSymbol, LinTermSymbolResolution)] {
        self.symbol_resolutions.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }
}

impl LinkageInstantiation {
    /// a nondeterminstic map basically
    pub(crate) fn from_javelin(
        javelin_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        let mut linkage_instantiations = smallvec![];
        Self::from_javelin_aux(
            javelin_instantiation,
            LinkageInstantiation {
                symbol_resolutions: Default::default(),
                separator: javelin_instantiation.separator,
            },
            &mut linkage_instantiations,
            db,
        );
        linkage_instantiations
    }

    fn from_javelin_aux(
        javelin_instantiation: &JavInstantiation,
        prefix: LinkageInstantiation,
        linkage_instantiations: &mut SmallVec<[Self; 4]>,
        db: &::salsa::Db,
    ) {
        if prefix.len() == javelin_instantiation.len() {
            linkage_instantiations.push(prefix);
            return;
        }
        let (symbol, javelin_resolution) =
            javelin_instantiation.symbol_resolutions.data()[prefix.len()];
        let linkage_resolutions =
            LinTermSymbolResolution::from_javelin(javelin_resolution, &prefix, db);
        for linkage_resolution in linkage_resolutions {
            let mut prefix = prefix.clone();
            unsafe {
                prefix
                    .symbol_resolutions
                    .insert_new_unchecked((symbol, linkage_resolution))
            };
            Self::from_javelin_aux(javelin_instantiation, prefix, linkage_instantiations, db)
        }
    }
}

impl LinTermSymbolResolution {
    fn from_javelin(
        javelin_resolution: JavTermSymbolResolution,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        match javelin_resolution {
            JavTermSymbolResolution::Explicit(arg) => match arg {
                JavTemplateArgument::Vacant => todo!(),
                JavTemplateArgument::Type(javelin_ty) => {
                    smallvec![LinTermSymbolResolution::Explicit(
                        LinTemplateArgument::Type(LinType::from_javelin(
                            javelin_ty,
                            linkage_instantiation,
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
                    LinTermSymbolResolution::SelfPlace(LinPlace::Ref),
                    LinTermSymbolResolution::SelfPlace(LinPlace::RefMut),
                ]
            }
        }
    }

    fn from_hir(
        resolution: HirTermSymbolResolution,
        linkage_instantiation: &LinkageInstantiation,
        db: &salsa::Db,
    ) -> LinTermSymbolResolution {
        match resolution {
            HirTermSymbolResolution::Explicit(arg) => LinTermSymbolResolution::Explicit(
                LinTemplateArgument::from_hir(arg, Some(linkage_instantiation), db),
            ),
            HirTermSymbolResolution::SelfLifetime => LinTermSymbolResolution::SelfLifetime,
            HirTermSymbolResolution::SelfPlace(_) => todo!(),
        }
    }
}

pub trait LinkageInstantiate {
    type Output;

    fn linkage_instantiate(
        self,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self::Output;
}
