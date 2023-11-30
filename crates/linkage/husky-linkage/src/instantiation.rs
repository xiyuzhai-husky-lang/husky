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
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> LinkageInstantiation {
        LinkageInstantiation {
            symbol_resolutions: hir_instantiation
                .symbol_map()
                .iter()
                .map(|&(symbol, resolution)| {
                    (
                        symbol,
                        LinkageTermSymbolResolution::from_hir(
                            resolution,
                            linkage_instantiation,
                            db,
                        ),
                    )
                })
                .collect(),
            separator: hir_instantiation.separator(),
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
    fn from_hir(
        resolution: HirTermSymbolResolution,
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match resolution {
            HirTermSymbolResolution::Explicit(template_argument) => {
                LinkageTermSymbolResolution::Explicit(LinkageTemplateArgument::from_hir(
                    template_argument,
                    linkage_instantiation,
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
