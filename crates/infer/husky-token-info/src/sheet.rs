use husky_text::TextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenInfoSheet {
    token_infos: Vec<TokenInfo>,
}

impl std::ops::Index<TokenIdx> for TokenInfoSheet {
    type Output = TokenInfo;

    fn index(&self, index: TokenIdx) -> &Self::Output {
        &self.token_infos[index.raw()]
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
        assert_eq!(self.token_infos[token_idx.raw()], TokenInfo::None);
        self.token_infos[token_idx.raw()] = token_info
    }

    pub fn informative_tokens<'a>(
        &'a self,
        ranged_token_sheet: &'a RangedTokenSheet,
        token_sheet_data: &'a TokenSheetData,
    ) -> impl Iterator<Item = (&'a TokenInfo, (&'a TextRange, &'a Token))> + 'a {
        assert_eq!(self.token_infos.len(), ranged_token_sheet.len());
        assert_eq!(self.token_infos.len(), token_sheet_data.len());
        std::iter::zip(
            self.token_infos.iter(),
            std::iter::zip(
                ranged_token_sheet.token_ranges().iter(),
                token_sheet_data.tokens().iter(),
            ),
        )
    }
}
