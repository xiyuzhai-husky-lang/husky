use crate::*;

pub struct RegionalTokensData<'a> {
    tokens_data: &'a [TokenData],
}

impl<'a> RegionalTokensData<'a> {
    pub fn new(tokens_data: &'a [TokenData]) -> Self {
        Self { tokens_data }
    }
}

impl<'a> std::ops::Index<RegionalTokenIdx> for RegionalTokensData<'a> {
    type Output = TokenData;

    fn index(&self, regional_token_idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens_data[regional_token_idx.index()]
    }
}
