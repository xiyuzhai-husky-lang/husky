use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirTemplateSymbol, HirType,
};
use husky_javelin::instantiation::{JavelinInstantiation, JavelinTermSymbolResolution};
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

    pub(crate) fn new_ad_hoc() -> LinkageInstantiation {
        todo!()
    }

    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> LinkageInstantiation {
        LinkageInstantiation {
            symbol_resolutions: SmallVecMap::from_iter(hir_instantiation.symbol_map().iter().map(
                |&(symbol, resolution)| {
                    (
                        symbol,
                        LinkageTermSymbolResolution::from_hir(
                            resolution,
                            linkage_instantiation,
                            db,
                        ),
                    )
                },
            )),
            separator: hir_instantiation.separator(),
        }
    }

    #[track_caller]
    pub(crate) fn resolve(&self, symbol: HirTemplateSymbol) -> LinkageTermSymbolResolution {
        self.symbol_resolutions[symbol].1
    }
}

impl LinkageInstantiation {
    pub(crate) fn from_javelin(
        javelin_instantiation: &JavelinInstantiation,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        let mut linkage_instantiations = smallvec![];
        Self::from_javelin_aux(
            javelin_instantiation,
            Default::default(),
            &mut linkage_instantiations,
            db,
        );
        linkage_instantiations
    }

    fn from_javelin_aux(
        javelin_instantiation: &JavelinInstantiation,
        prefix: SmallVec<[(HirTemplateSymbol, LinkageTermSymbolResolution); 4]>,
        linkage_instantiations: &mut SmallVec<[Self; 4]>,
        db: &::salsa::Db,
    ) {
        if prefix.len() == linkage_instantiations.len() {
            linkage_instantiations.push(Self {
                symbol_resolutions: unsafe { SmallVecMap::from_smallvec_unchecked(prefix) },
                separator: javelin_instantiation.separator,
            });
            return;
        }
        let (symbol, javelin_resolution) =
            javelin_instantiation.symbol_resolutions.data()[prefix.len()];
        let linkage_resolutions = LinkageTermSymbolResolution::from_javelin(javelin_resolution, db);
        todo!()
    }
}

impl LinkageTermSymbolResolution {
    fn from_javelin(
        javelin_resolution: JavelinTermSymbolResolution,
        db: &::salsa::Db,
    ) -> SmallVec<[Self; 4]> {
        match javelin_resolution {
            JavelinTermSymbolResolution::Explicit(_) => todo!(),
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
