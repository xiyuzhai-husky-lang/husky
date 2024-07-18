use super::*;
use crate::place_contract_site::HirPlaceContractSite;
use crate::quary::HirContractedQuary;
use context::HirTypeContext;
use husky_eth_term::instantiation::EthInstantiation;
use husky_fly_term::{
    instantiation::{FlyInstantiation, FlyTermSymbolResolution},
    FlyTerms,
};
use vec_like::{SmallVecMap, SmallVecPairMap};

/// `HirInstantiation` maps each hir symbol to its hir resolution.
///
/// hir resolution might still contain hir symbols.
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirInstantiation {
    context: HirTypeContext,
    symbol_map: SmallVecPairMap<HirTemplateVariable, HirTermSymbolicVariableResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for HirInstantiation {
    type Target = [(HirTemplateVariable, HirTermSymbolicVariableResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_map
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTermSymbolicVariableResolution {
    Explicit(HirTemplateArgument),
    /// means we don't care about it now
    SelfLifetime,
    SelfContractedQuary(HirContractedQuary),
}
impl HirTermSymbolicVariableResolution {
    fn is_univalent_for_javelin(&self) -> bool {
        match self {
            HirTermSymbolicVariableResolution::Explicit(arg) => match arg {
                HirTemplateArgument::Vacant => true,
                HirTemplateArgument::Type(_) => false,
                HirTemplateArgument::Constant(_) => false,
                HirTemplateArgument::Lifetime(_) => true,
                HirTemplateArgument::ContractedQuary(_) => true,
            },
            HirTermSymbolicVariableResolution::SelfLifetime => true,
            HirTermSymbolicVariableResolution::SelfContractedQuary(_) => true,
        }
    }
}

impl HirInstantiation {
    pub fn from_fly(
        instantiation: &FlyInstantiation,
        place_contract_site: &HirPlaceContractSite,
        db: &::salsa::Db,
        terms: &FlyTerms,
    ) -> Self {
        let (symbol_map0, symbol_map1) = &instantiation.symbol_map_splitted();
        let t = |&(symbol, resolution)| match HirTemplateVariable::from_eth(symbol, db) {
            Some(symbol) => Some((
                symbol,
                match resolution {
                    FlyTermSymbolResolution::Explicit(term) => {
                        HirTermSymbolicVariableResolution::Explicit(
                            HirTemplateArgument::from_fly(term, db, terms).expect("some"),
                        )
                    }
                    FlyTermSymbolResolution::SelfLifetime => {
                        HirTermSymbolicVariableResolution::SelfLifetime
                    }
                    FlyTermSymbolResolution::SelfQuary(quary) => {
                        HirTermSymbolicVariableResolution::SelfContractedQuary(
                            HirContractedQuary::from_fly(quary, place_contract_site),
                        )
                    }
                },
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<
            (HirTemplateVariable, HirTermSymbolicVariableResolution),
            4,
        > = symbol_map0.iter().filter_map(t).collect();
        let mut separator: Option<u8> = None;
        match symbol_map1 {
            Some(symbol_map1) => {
                separator = Some(symbol_map.len().try_into().unwrap());
                symbol_map.extend(symbol_map1.iter().filter_map(t)).unwrap()
            }
            None => (),
        }
        Self {
            symbol_map,
            separator,
            context: HirTypeContext::from_fly(instantiation, db),
        }
    }

    pub fn from_eth(eth_instantiation: &EthInstantiation, db: &::salsa::Db) -> Self {
        let (symbol_map0, symbol_map1) = &eth_instantiation.symbol_map_splitted();
        let t = |&(symbol, term)| match HirTemplateVariable::from_eth(symbol, db) {
            Some(symbol) => Some((
                symbol,
                HirTermSymbolicVariableResolution::Explicit(
                    HirTemplateArgument::from_eth(term, db).expect("some"),
                ),
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<
            (HirTemplateVariable, HirTermSymbolicVariableResolution),
            4,
        > = symbol_map0.iter().filter_map(t).collect();
        let mut separator: Option<u8> = None;
        match symbol_map1 {
            Some(symbol_map1) => {
                separator = Some(symbol_map.len().try_into().unwrap());
                symbol_map.extend(symbol_map1.iter().filter_map(t)).unwrap()
            }
            None => (),
        }
        Self {
            symbol_map,
            separator,
            context: HirTypeContext::from_eth(eth_instantiation, db),
        }
    }
}

impl HirInstantiation {
    pub fn context(&self) -> &HirTypeContext {
        &self.context
    }

    pub fn symbol_map(&self) -> &[(HirTemplateVariable, HirTermSymbolicVariableResolution)] {
        self.symbol_map.as_ref()
    }

    pub fn contracted_quaries(&self) -> SmallVec<[HirContractedQuary; 2]> {
        self.symbol_map
            .iter()
            .filter_map(|&(_, res)| match res {
                HirTermSymbolicVariableResolution::Explicit(_) => None,
                HirTermSymbolicVariableResolution::SelfLifetime => None,
                HirTermSymbolicVariableResolution::SelfContractedQuary(quary) => Some(quary),
            })
            .collect()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn is_univalent_for_javelin(&self) -> bool {
        self.symbol_map
            .iter()
            .all(|(_, res)| res.is_univalent_for_javelin())
    }
}
