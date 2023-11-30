use crate::{
    template_argument::{place::JavelinPlace, JavelinTemplateArgument},
    *,
};
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirComptimeSymbol,
};
use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavelinInstantiation {
    symbol_resolutions: SmallVecPairMap<HirComptimeSymbol, JavelinTermSymbolResolution, 4>,
    separator: Option<u8>,
}
impl JavelinInstantiation {
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> JavelinInstantiation {
        JavelinInstantiation {
            symbol_resolutions: hir_instantiation
                .symbol_map()
                .iter()
                .map(|&(symbol, resolution)| {
                    (
                        symbol,
                        JavelinTermSymbolResolution::from_hir(
                            resolution,
                            javelin_instantiation,
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
pub enum JavelinTermSymbolResolution {
    Explicit(JavelinTemplateArgument),
    SelfLifetime,
    SelfPlace(JavelinPlace),
}

impl JavelinTermSymbolResolution {
    fn from_hir(
        resolution: HirTermSymbolResolution,
        javelin_instantiation: Option<&JavelinInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match resolution {
            HirTermSymbolResolution::Explicit(template_argument) => {
                JavelinTermSymbolResolution::Explicit(JavelinTemplateArgument::from_hir(
                    template_argument,
                    javelin_instantiation,
                    db,
                ))
            }
            HirTermSymbolResolution::SelfLifetime => JavelinTermSymbolResolution::SelfLifetime,
            HirTermSymbolResolution::SelfPlace(place) => {
                JavelinTermSymbolResolution::SelfPlace(JavelinPlace::from_hir(place))
            }
        }
    }
}
