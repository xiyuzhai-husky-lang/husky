use crate::{
    context::JavTypeContext,
    template_argument::{ty::JavType, JavTemplateArgument},
};
use husky_entity_path::path::ItemPath;
use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSymbolicVariableResolution},
    HirTemplateVariable, HirTemplateVariableClass,
};
use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavInstantiation {
    path: ItemPath,
    context: JavTypeContext,
    // todo: getters
    pub variable_resolutions: SmallVecPairMap<HirTemplateVariable, JavTermSymbolResolution, 4>,
    pub separator: Option<u8>,
}

impl std::ops::Deref for JavInstantiation {
    type Target = [(HirTemplateVariable, JavTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.variable_resolutions
    }
}

impl JavInstantiation {
    pub(crate) fn from_hir(
        hir_instantiation: &HirInstantiation,
        jav_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> JavInstantiation {
        JavInstantiation {
            path: hir_instantiation.path(),
            context: JavTypeContext::from_hir(hir_instantiation.context(), jav_instantiation, db),
            variable_resolutions: hir_instantiation
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
                        JavTermSymbolResolution::from_hir(resolution, jav_instantiation, db),
                    ))
                })
                .collect(),
            separator: hir_instantiation.separator(),
        }
    }

    pub(crate) fn new_amazon(path: ItemPath, db: &::salsa::Db) -> Self {
        Self {
            path,
            context: JavTypeContext::new_amazon(path, db),
            variable_resolutions: Default::default(),
            separator: match path {
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
}

impl JavInstantiation {
    pub fn path(&self) -> ItemPath {
        self.path
    }

    pub fn is_univalent(&self) -> bool {
        self.variable_resolutions
            .iter()
            .all(|(_, res)| res.is_univalent())
    }

    pub fn resolve(&self, symbol: impl Into<HirTemplateVariable>) -> JavTermSymbolResolution {
        self.variable_resolutions[symbol.into()].1
    }

    #[track_caller]
    pub fn resolve_ty(&self, symbol: impl Into<HirTemplateVariable>) -> JavType {
        match self.variable_resolutions[symbol.into()].1 {
            JavTermSymbolResolution::Explicit(JavTemplateArgument::Type(ty)) => ty,
            _ => unreachable!("expect type"),
        }
    }

    pub fn context(&self) -> &JavTypeContext {
        &self.context
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
    pub(crate) fn from_hir(
        resolution: HirTermSymbolicVariableResolution,
        jav_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match resolution {
            HirTermSymbolicVariableResolution::Explicit(template_argument) => {
                JavTermSymbolResolution::Explicit(JavTemplateArgument::from_hir(
                    template_argument,
                    jav_instantiation,
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
