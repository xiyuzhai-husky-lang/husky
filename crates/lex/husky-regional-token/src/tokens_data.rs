use crate::*;

#[derive(Clone, Copy)]
pub struct RegionalTokensData<'a> {
    tokens_data: &'a [TokenData],
}

impl<'a> RegionalTokensData<'a> {
    pub fn new(tokens_data: &'a [TokenData]) -> Self {
        Self { tokens_data }
    }

    pub fn len(self) -> usize {
        self.tokens_data.len()
    }
}

impl<'a> std::ops::Index<RegionalTokenIdx> for RegionalTokensData<'a> {
    type Output = TokenData;

    fn index(&self, regional_token_idx: RegionalTokenIdx) -> &Self::Output {
        &self.tokens_data[regional_token_idx.index()]
    }
}
