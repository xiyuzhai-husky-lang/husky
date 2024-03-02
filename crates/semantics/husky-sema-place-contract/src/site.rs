use husky_place::place::Place;
use husky_term_prelude::TermContract;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SemaPlaceContractSite {}
impl SemaPlaceContractSite {
    pub(crate) fn set(&mut self, place: Place, contract: TermContract) {
        todo!()
    }
}
