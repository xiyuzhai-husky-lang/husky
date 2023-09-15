use husky_regional_token::RegionalTokenIdx;
use husky_text::TextRange;
use husky_token_data::TokenData;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb)]
pub struct TokenInfoSheet {
    token_infos: Vec<TokenInfo>,
}

impl std::ops::Index<TokenIdx> for TokenInfoSheet {
    type Output = TokenInfo;

    fn index(&self, idx: TokenIdx) -> &Self::Output {
        &self.token_infos[idx.index()]
    }
}

impl TokenInfoSheet {
    pub(crate) fn new(token_sheet: &TokenSheetData) -> Self {
        TokenInfoSheet {
            token_infos: (0..token_sheet.len())
                .into_iter()
                .map(|_| TokenInfo::default())
                .collect(),
        }
    }

    pub(crate) fn add(&mut self, token_idx: TokenIdx, token_info: TokenInfo) {
        assert_eq!(self.token_infos[token_idx.index()], TokenInfo::None);
        self.token_infos[token_idx.index()] = token_info
    }

    pub fn informative_ranged_token_iter<'a>(
        &'a self,
        ranged_token_sheet: &'a RangedTokenSheet,
        db: &'a dyn TokenInfoDb,
    ) -> impl Iterator<Item = (&'a TokenInfo, (&'a TextRange, &'a TokenData))> + 'a {
        std::iter::zip(
            self.token_infos.iter(),
            ranged_token_sheet.ranged_token_iter(db),
        )
    }
}
