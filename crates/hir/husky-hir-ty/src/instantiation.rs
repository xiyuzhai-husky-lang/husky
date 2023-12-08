use super::*;
use crate::place::HirPlace;
use husky_fluffy_term::{
    instantiation::{FluffyInstantiation, FluffyTermSymbolResolution},
    FluffyTerms,
};
use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
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
    pub fn new_empty() -> Self {
        Self {
            symbol_map: Default::default(),
            separator: None,
        }
    }

    pub fn from_fluffy(
        fluffy_instantiation: &FluffyInstantiation,
        db: &::salsa::Db,
        fluffy_terms: &FluffyTerms,
    ) -> Self {
        Self {
            symbol_map: fluffy_instantiation
                .symbol_map()
                .iter()
                .filter_map(|&(symbol, resolution)| {
                    match HirTemplateSymbol::from_ethereal(symbol, db) {
                        Some(symbol) => Some((
                            symbol,
                            match resolution {
                                FluffyTermSymbolResolution::Explicit(fluffy_term) => {
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::from_fluffy(
                                            fluffy_term,
                                            db,
                                            fluffy_terms,
                                        )
                                        .expect("some"),
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
                    }
                })
                .collect(),
            separator: fluffy_instantiation.separator(),
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
