use super::*;
use crate::place::HirPlace;
use husky_ethereal_term::instantiation::EtherealInstantiation;
use husky_fluffy_term::{
    instantiation::{FluffyInstantiation, FluffyTermSymbolResolution},
    FluffyTerms,
};
use vec_like::{SmallVecMap, SmallVecPairMap};

/// `HirInstantiation` maps each hir symbol to its hir resolution.
///
/// hir resolution might still contain hir symbols.
#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirInstantiation {
    symbol_map: SmallVecPairMap<HirTemplateSymbol, HirTermSymbolResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for HirInstantiation {
    type Target = [(HirTemplateSymbol, HirTermSymbolResolution)];

    fn deref(&self) -> &Self::Target {
        &self.symbol_map
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTermSymbolResolution {
    Explicit(HirTemplateArgument),
    /// means we don't care about it now
    SelfLifetime,
    SelfPlace(HirPlace),
}
impl HirTermSymbolResolution {
    fn is_univalent_for_javelin(&self) -> bool {
        match self {
            HirTermSymbolResolution::Explicit(arg) => match arg {
                HirTemplateArgument::Vacant => true,
                HirTemplateArgument::Type(_) => false,
                HirTemplateArgument::Constant(_) => false,
                HirTemplateArgument::Lifetime(_) => true,
                HirTemplateArgument::Place(_) => true,
            },
            HirTermSymbolResolution::SelfLifetime => true,
            HirTermSymbolResolution::SelfPlace(_) => true,
        }
    }
}

impl HirInstantiation {
    #[deprecated]
    pub fn new_empty(is_associated: bool) -> Self {
        Self {
            symbol_map: Default::default(),
            separator: is_associated.then_some(0),
        }
    }

    pub fn from_fluffy(
        instantiation: &FluffyInstantiation,
        db: &::salsa::Db,
        terms: &FluffyTerms,
    ) -> Self {
        let (symbol_map0, symbol_map1) = &instantiation.symbol_map_splitted();
        let t = |&(symbol, resolution)| match HirTemplateSymbol::from_ethereal(symbol, db) {
            Some(symbol) => Some((
                symbol,
                match resolution {
                    FluffyTermSymbolResolution::Explicit(term) => {
                        HirTermSymbolResolution::Explicit(
                            HirTemplateArgument::from_fluffy(term, db, terms).expect("some"),
                        )
                    }
                    FluffyTermSymbolResolution::SelfLifetime => {
                        HirTermSymbolResolution::SelfLifetime
                    }
                    FluffyTermSymbolResolution::SelfPlace(place) => {
                        HirTermSymbolResolution::SelfPlace(HirPlace::from_fluffy(place))
                    }
                },
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<(HirTemplateSymbol, HirTermSymbolResolution), 4> =
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

    pub fn from_ethereal(ethereal_instantiation: &EtherealInstantiation, db: &::salsa::Db) -> Self {
        let (symbol_map0, symbol_map1) = &ethereal_instantiation.symbol_map_splitted();
        let t = |&(symbol, term)| match HirTemplateSymbol::from_ethereal(symbol, db) {
            Some(symbol) => Some((
                symbol,
                HirTermSymbolResolution::Explicit(
                    HirTemplateArgument::from_ethereal(term, db).expect("some"),
                ),
            )),
            None => None,
        };
        let mut symbol_map: SmallVecMap<(HirTemplateSymbol, HirTermSymbolResolution), 4> =
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

    pub fn symbol_map(&self) -> &[(HirTemplateSymbol, HirTermSymbolResolution)] {
        self.symbol_map.as_ref()
    }

    pub fn places(&self) -> SmallVec<[HirPlace; 2]> {
        self.symbol_map
            .iter()
            .filter_map(|&(_, res)| match res {
                HirTermSymbolResolution::Explicit(_) => None,
                HirTermSymbolResolution::SelfLifetime => None,
                HirTermSymbolResolution::SelfPlace(place) => Some(place),
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
