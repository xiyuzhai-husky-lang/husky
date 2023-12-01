use crate::{template_argument::JavelinTemplateArgument, *};
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    HirComptimeSymbol,
};
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
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

    pub(crate) fn new_empty(/* places */) -> Self {
        Self {
            symbol_resolutions: Default::default(),
            separator: None,
        }
    }

    pub fn is_univalent(&self) -> bool {
        self.symbol_resolutions
            .iter()
            .all(|(_, res)| res.is_univalent())
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JavelinTermSymbolResolution {
    Explicit(JavelinTemplateArgument),
    SelfLifetime,
    SelfPlace,
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
            HirTermSymbolResolution::SelfPlace(_) => JavelinTermSymbolResolution::SelfPlace,
        }
    }

    fn is_univalent(&self) -> bool {
        match self {
            JavelinTermSymbolResolution::Explicit(arg) => match arg {
                JavelinTemplateArgument::Vacant => true,
                JavelinTemplateArgument::Type(_) => false,
                JavelinTemplateArgument::Constant(_) => false,
                JavelinTemplateArgument::Lifetime => true,
                JavelinTemplateArgument::Place => true,
            },
            JavelinTermSymbolResolution::SelfLifetime => true,
            JavelinTermSymbolResolution::SelfPlace => true,
        }
    }
}
