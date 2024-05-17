use crate::ritchie::HirContract;
use husky_place::place::EthPlace;
use husky_sem_place_contract::site::SemPlaceContractSite;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirPlaceContractSite {
    place_contracts: SmallVecPairMap<EthPlace, HirContract, 2>,
}

impl HirPlaceContractSite {
    pub fn from_sema(sem_site: &SemPlaceContractSite) -> Self {
        HirPlaceContractSite {
            place_contracts: SmallVecPairMap::from_iter(
                sem_site
                    .place_contracts()
                    .iter()
                    .copied()
                    .filter_map(|(place, contract)| {
                        (contract != Contract::At)
                            .then_some((place, HirContract::from_contract(contract)))
                    }),
            ),
        }
    }
}

impl HirPlaceContractSite {
    pub fn place_contracts(&self) -> &[(EthPlace, HirContract)] {
        &self.place_contracts
    }

    pub fn get(&self, place: EthPlace) -> Option<HirContract> {
        self.place_contracts.get_value(place).copied()
    }
}

impl std::ops::Index<EthPlace> for HirPlaceContractSite {
    type Output = HirContract;

    fn index(&self, place: EthPlace) -> &Self::Output {
        &self.place_contracts[place].1
    }
}
