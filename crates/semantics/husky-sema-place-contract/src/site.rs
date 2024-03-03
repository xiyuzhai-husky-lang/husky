use husky_place::place::Place;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SemaPlaceContractSite {
    place_contracts: SmallVecPairMap<Place, Contract, 2>,
}

impl SemaPlaceContractSite {
    pub(crate) fn set(&mut self, place: Place, contract: Contract) {
        self.place_contracts.update_value_or_insert(
            place,
            |old_contract| *old_contract *= contract,
            contract,
        )
    }

    pub fn place_contracts(&self) -> &[(Place, Contract)] {
        &self.place_contracts
    }
}
