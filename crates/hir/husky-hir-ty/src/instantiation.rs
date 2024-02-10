use super::*;
use crate::place::HirPlace;
use husky_eth_term::instantiation::EthInstantiation;
use husky_fly_term::{
    instantiation::{FlyInstantiation, FlyTermSymbolResolution},
    FlyTerms,
};
use vec_like::{SmallVecMap, SmallVecPairMap};

/// `HirInstantiation` maps each hir symbol to its hir resolution.
///
/// hir resolution might still contain hir symbols.
#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirInstantiation {
    symbol_map: SmallVecPairMap<HirTemplateVar, HirTermSvarResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for HirInstantiation {
    type Target = [(HirTemplateVar, HirTermSvarResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_map
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTermSvarResolution {
    Explicit(HirTemplateArgument),
    /// means we don't care about it now
    SelfLifetime,
    SelfPlace(HirPlace),
}
impl HirTermSvarResolution {
    fn is_univalent_for_javelin(&self) -> bool {
        match self {
            HirTermSvarResolution::Explicit(arg) => match arg {
                HirTemplateArgument::Vacant => true,
                HirTemplateArgument::Type(_) => false,
                HirTemplateArgument::Constant(_) => false,
                HirTemplateArgument::Lifetime(_) => true,
                HirTemplateArgument::Place(_) => true,
            },
            HirTermSvarResolution::SelfLifetime => true,
            HirTermSvarResolution::SelfPlace(_) => true,
        }
    }
}

impl HirInstantiation {
    pub fn from_fly(instantiation: &FlyInstantiation, db: &::salsa::Db, terms: &FlyTerms) -> Self {
        let (symbol_map0, symbol_map1) = &instantiation.symbol_map_splitted();
        let t = |&(symbol, resolution)| match HirTemplateVar::from_eth(symbol, db) {
            Some(symbol) => Some((
                symbol,
                match resolution {
                    FlyTermSymbolResolution::Explicit(term) => HirTermSvarResolution::Explicit(
                        HirTemplateArgument::from_fly(term, db, terms).expect("some"),
                    ),
                    FlyTermSymbolResolution::SelfLifetime => HirTermSvarResolution::SelfLifetime,
                    FlyTermSymbolResolution::SelfPlace(place) => {
                        HirTermSvarResolution::SelfPlace(HirPlace::from_fly(place))
                    }
                },
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<(HirTemplateVar, HirTermSvarResolution), 4> =
            symbol_map0.iter().filter_map(t).collect();
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
        }
    }

    pub fn from_eth(ethereal_instantiation: &EthInstantiation, db: &::salsa::Db) -> Self {
        let (symbol_map0, symbol_map1) = &ethereal_instantiation.symbol_map_splitted();
        let t = |&(symbol, term)| match HirTemplateVar::from_eth(symbol, db) {
            Some(symbol) => Some((
                symbol,
                HirTermSvarResolution::Explicit(
                    HirTemplateArgument::from_eth(term, db).expect("some"),
                ),
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<(HirTemplateVar, HirTermSvarResolution), 4> =
            symbol_map0.iter().filter_map(t).collect();
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
        }
    }

    pub fn symbol_map(&self) -> &[(HirTemplateVar, HirTermSvarResolution)] {
        self.symbol_map.as_ref()
    }

    pub fn places(&self) -> SmallVec<[HirPlace; 2]> {
        self.symbol_map
            .iter()
            .filter_map(|&(_, res)| match res {
                HirTermSvarResolution::Explicit(_) => None,
                HirTermSvarResolution::SelfLifetime => None,
                HirTermSvarResolution::SelfPlace(place) => Some(place),
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
