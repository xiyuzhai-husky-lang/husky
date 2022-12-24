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
}
