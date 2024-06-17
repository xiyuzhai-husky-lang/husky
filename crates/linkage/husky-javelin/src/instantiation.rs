use crate::template_argument::{ty::JavelinType, JavTemplateArgument};
use husky_entity_path::path::ItemPath;
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolicVariableResolution},
    HirTemplateVariable, HirTemplateVariableClass,
};
use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavInstantiation {
    pub symbol_resolutions: SmallVecPairMap<HirTemplateVariable, JavTermSymbolResolution, 4>,
    pub separator: Option<u8>,
}

impl std::ops::Deref for JavInstantiation {
    type Target = [(HirTemplateVariable, JavTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_resolutions
    }
}

impl JavInstantiation {
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        javelin_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> JavInstantiation {
        JavInstantiation {
            symbol_resolutions: hir_instantiation
                .symbol_map()
                .iter()
                .filter_map(|&(symbol, resolution)| {
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
                        JavTermSymbolResolution::from_hir(resolution, javelin_instantiation, db),
                    ))
                })
                .collect(),
            separator: hir_instantiation.separator(),
        }
    }

    pub(crate) fn new_amazon(item_path: ItemPath) -> Self {
        Self {
            symbol_resolutions: Default::default(),
            separator: match item_path {
                ItemPath::Submodule(_, _) => todo!(),
                ItemPath::MajorItem(_) => None,
                ItemPath::AssocItem(_) => Some(0),
                ItemPath::TypeVariant(_, _) => None,
                ItemPath::ImplBlock(_) => todo!(),
                ItemPath::Attr(_, _) => todo!(),
                ItemPath::Chunk(_, _) => todo!(),
            },
        }
    }

    pub fn is_univalent(&self) -> bool {
        self.symbol_resolutions
            .iter()
            .all(|(_, res)| res.is_univalent())
    }

    pub fn resolve(&self, symbol: impl Into<HirTemplateVariable>) -> JavTermSymbolResolution {
        self.symbol_resolutions[symbol.into()].1
    }

    #[track_caller]
    pub fn resolve_ty(&self, symbol: impl Into<HirTemplateVariable>) -> JavelinType {
        match self.symbol_resolutions[symbol.into()].1 {
            JavTermSymbolResolution::Explicit(JavTemplateArgument::Type(ty)) => ty,
            _ => unreachable!("expect type"),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JavTermSymbolResolution {
    Explicit(JavTemplateArgument),
    SelfLifetime,
    SelfPlace,
}

impl JavTermSymbolResolution {
    fn from_hir(
        resolution: HirTermSymbolicVariableResolution,
        javelin_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match resolution {
            HirTermSymbolicVariableResolution::Explicit(template_argument) => {
                JavTermSymbolResolution::Explicit(JavTemplateArgument::from_hir(
                    template_argument,
                    javelin_instantiation,
                    db,
                ))
            }
            HirTermSymbolicVariableResolution::SelfLifetime => {
                JavTermSymbolResolution::SelfLifetime
            }
            HirTermSymbolicVariableResolution::SelfContractedQuary(_) => {
                JavTermSymbolResolution::SelfPlace
            }
        }
    }

    fn is_univalent(&self) -> bool {
        match self {
            JavTermSymbolResolution::Explicit(arg) => match arg {
                JavTemplateArgument::Vacant => true,
                JavTemplateArgument::Type(_) => false,
                JavTemplateArgument::Constant(_) => false,
                JavTemplateArgument::Lifetime => true,
                JavTemplateArgument::Place => true,
            },
            JavTermSymbolResolution::SelfLifetime => true,
            JavTermSymbolResolution::SelfPlace => true,
        }
    }
}
