use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirTemplateSymbol, HirType,
};
use husky_javelin::{
    instantiation::{JavelinInstantiation, JavelinTermSymbolResolution},
    template_argument::JavelinTemplateArgument,
};
use smallvec::*;
use vec_like::{SmallVecMap, SmallVecPairMap};

use crate::template_argument::{
    place::LinkagePlace,
    ty::{LinkageType, LinkageTypePathLeading},
    LinkageTemplateArgument,
};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LinkageInstantiation {
    symbol_resolutions: SmallVecPairMap<HirTemplateSymbol, LinkageTermSymbolResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for LinkageInstantiation {
    type Target = [(HirTemplateSymbol, LinkageTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_resolutions
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkageTermSymbolResolution {
    Explicit(LinkageTemplateArgument),
    SelfLifetime,
    SelfPlace(LinkagePlace),
}

impl LinkageInstantiation {
    pub fn new_empty() -> Self {
        LinkageInstantiation {
            symbol_resolutions: Default::default(),
            separator: None,
        }
    }

    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> LinkageInstantiation {
        let symbol_resolutions = SmallVecMap::from_iter(hir_instantiation.symbol_map().iter().map(
            |&(symbol, resolution)| {
                (
                    symbol,
                    LinkageTermSymbolResolution::from_hir(resolution, linkage_instantiation, db),
                )
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
    pub(crate) fn resolve(&self, symbol: HirTemplateSymbol) -> LinkageTermSymbolResolution {
        self.symbol_resolutions[symbol].1
    }
}

impl LinkageInstantiation {
    /// a nondeterminstic map basically
    pub(crate) fn from_javelin(
        javelin_instantiation: &JavelinInstantiation,
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
        javelin_instantiation: &JavelinInstantiation,
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
            LinkageTermSymbolResolution::from_javelin(javelin_resolution, &prefix, db);
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

impl LinkageTermSymbolResolution {
    fn from_javelin(
        javelin_resolution: JavelinTermSymbolResolution,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        match javelin_resolution {
            JavelinTermSymbolResolution::Explicit(arg) => match arg {
                JavelinTemplateArgument::Vacant => todo!(),
                JavelinTemplateArgument::Type(javelin_ty) => {
                    smallvec![LinkageTermSymbolResolution::Explicit(
                        LinkageTemplateArgument::Type(LinkageType::from_javelin(
                            javelin_ty,
                            linkage_instantiation,
                            db
                        ))
                    )]
                }
                JavelinTemplateArgument::Constant(_) => todo!(),
                JavelinTemplateArgument::Lifetime => todo!(),
                JavelinTemplateArgument::Place => todo!(),
            },
            JavelinTermSymbolResolution::SelfLifetime => {
                smallvec![LinkageTermSymbolResolution::SelfLifetime]
            }
            JavelinTermSymbolResolution::SelfPlace => {
                smallvec![
                    LinkageTermSymbolResolution::SelfPlace(LinkagePlace::Ref),
                    LinkageTermSymbolResolution::SelfPlace(LinkagePlace::RefMut),
                ]
            }
        }
    }

    fn from_hir(
        resolution: HirTermSymbolResolution,
        linkage_instantiation: &LinkageInstantiation,
        db: &salsa::Db,
    ) -> LinkageTermSymbolResolution {
        match resolution {
            HirTermSymbolResolution::Explicit(arg) => LinkageTermSymbolResolution::Explicit(
                LinkageTemplateArgument::from_hir(arg, Some(linkage_instantiation), db),
            ),
            HirTermSymbolResolution::SelfLifetime => LinkageTermSymbolResolution::SelfLifetime,
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
