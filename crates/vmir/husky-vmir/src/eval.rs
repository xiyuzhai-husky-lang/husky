use husky_place::place::EthPlace;

pub trait EvalVmir {
    type Value;

    // todo: change to LinPlace??
    fn borrow_variable(&self, place: EthPlace) -> Self::Value;
    fn borrow_variable_mut(&mut self, place: EthPlace) -> Self::Value;
}
