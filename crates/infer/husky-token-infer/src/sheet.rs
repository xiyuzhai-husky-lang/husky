use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenInferSheet {
    token_infos: Vec<TokenInfo>,
}

impl TokenInferSheet {
    pub(crate) fn new(token_sheet: &TokenSheet) -> Self {
        TokenInferSheet {
            token_infos: (0..token_sheet.len())
                .into_iter()
                .map(|_| TokenInfo::default())
                .collect(),
        }
    }

    pub fn informative_tokens<'a>(
        &'a self,
        token_sheet: &'a TokenSheet,
    ) -> impl Iterator<Item = (&'a TokenInfo, &'a Token)> + 'a {
        assert_eq!(self.token_infos.len(), token_sheet.tokens().len());
        std::iter::zip(self.token_infos.iter(), token_sheet.tokens().iter())
    }
}
