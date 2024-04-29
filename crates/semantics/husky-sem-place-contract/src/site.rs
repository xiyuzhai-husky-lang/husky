use husky_place::place::EthPlace;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SemPlaceContractSite {
    place_contracts: SmallVecPairMap<EthPlace, Contract, 2>,
}

impl SemPlaceContractSite {
    pub(crate) fn set(&mut self, place: EthPlace, contract: Contract) {
        self.place_contracts.update_value_or_insert(
            place,
            |old_contract| *old_contract *= contract,
            contract,
        )
    }

    pub fn place_contracts(&self) -> &[(EthPlace, Contract)] {
        &self.place_contracts
    }
}
