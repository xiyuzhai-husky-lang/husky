use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenInfoSheet {
    token_infos: Vec<TokenInfo>,
}

impl TokenInfoSheet {
    pub(crate) fn new(token_sheet: &TokenSheet) -> Self {
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
        token_sheet: &'a TokenSheet,
    ) -> impl Iterator<Item = (&'a TokenInfo, &'a Token)> + 'a {
        assert_eq!(self.token_infos.len(), token_sheet.tokens().len());
        std::iter::zip(self.token_infos.iter(), token_sheet.tokens().iter())
    }
}
