use husky_regional_token::RegionalTokenIdx;
use husky_sema_expr::SemaExprIdx;
use husky_text::TextRange;
use husky_token_data::TokenData;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb)]
pub struct TokenInfoSheet {
    token_infos: Vec<Option<TokenInfo>>,
}

impl TokenInfoSheet {
    pub fn to_ref(&self) -> TokenInfoSheetRef {
        TokenInfoSheetRef {
            token_infos: &self.token_infos,
        }
    }
}

impl std::ops::Index<TokenIdx> for TokenInfoSheet {
    type Output = Option<TokenInfo>;

    fn index(&self, idx: TokenIdx) -> &Self::Output {
        &self.token_infos[idx.index()]
    }
}

pub struct TokenInfoSheetRef<'a> {
    token_infos: &'a [Option<TokenInfo>],
}

impl<'a> std::ops::Index<TokenIdx> for TokenInfoSheetRef<'a> {
    type Output = Option<TokenInfo>;

    fn index(&self, idx: TokenIdx) -> &Self::Output {
        &self.token_infos[idx.index()]
    }
}

impl TokenInfoSheet {
    pub(crate) fn new(token_sheet: &TokenSheetData) -> Self {
        TokenInfoSheet {
            token_infos: (0..token_sheet.len()).into_iter().map(|_| None).collect(),
        }
    }

    pub(crate) fn add(
        &mut self,
        token_idx: TokenIdx,
        src: impl Into<TokenInfoSource>,
        token_info_data: TokenInfoData,
    ) {
        assert_eq!(self.token_infos[token_idx.index()], None);
        self.token_infos[token_idx.index()] = Some(TokenInfo::new(src, token_info_data))
    }

    pub(crate) fn override_add(
        &mut self,
        token_idx: TokenIdx,
        src: impl Into<TokenInfoSource>,
        token_info_data: TokenInfoData,
    ) {
        self.token_infos[token_idx.index()] = Some(TokenInfo::new(src, token_info_data))
    }

    pub fn informative_ranged_token_iter<'a>(
        &'a self,
        ranged_token_sheet: &'a RangedTokenSheet,
        db: &'a dyn TokenInfoDb,
    ) -> impl Iterator<Item = (Option<&'a TokenInfo>, (&'a TextRange, &'a TokenData))> + 'a {
        std::iter::zip(
            self.token_infos.iter(),
            ranged_token_sheet.ranged_token_iter(db),
        )
        .map(|(info, other)| (info.as_ref(), other))
    }
}
