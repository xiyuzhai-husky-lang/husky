use crate::*;
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirComptimeSymbol,
};
use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkageInstantiation {
    symbol_resolutions: SmallVecPairMap<HirComptimeSymbol, LinkageTermSymbolResolution, 4>,
    separator: Option<u8>,
}
impl LinkageInstantiation {
    pub(crate) fn from_hir(
        instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> LinkageInstantiation {
        LinkageInstantiation {
            symbol_resolutions: instantiation
                .symbol_map()
                .iter()
                .map(|&(symbol, resolution)| {
                    (
                        symbol,
                        LinkageTermSymbolResolution::from_hir(resolution, db),
                    )
                })
                .collect(),
            separator: instantiation.separator(),
        }
    }

    pub(crate) fn new_first_born(/* places */) -> Self {
        Self {
            symbol_resolutions: Default::default(),
            separator: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkageTermSymbolResolution {
    Explicit(LinkageTemplateArgument),
    SelfLifetime,
    SelfPlace(LinkagePlace),
}

impl LinkageTermSymbolResolution {
    fn from_hir(resolution: HirTermSymbolResolution, db: &::salsa::Db) -> Self {
        match resolution {
            HirTermSymbolResolution::Explicit(template_argument) => {
                LinkageTermSymbolResolution::Explicit(LinkageTemplateArgument::from_hir(
                    template_argument,
                    db,
                ))
            }
            HirTermSymbolResolution::SelfLifetime => LinkageTermSymbolResolution::SelfLifetime,
            HirTermSymbolResolution::SelfPlace(place) => {
                LinkageTermSymbolResolution::SelfPlace(LinkagePlace::from_hir(place))
            }
        }
    }
}
