mod refutability;
mod ty;

use crate::*;
use husky_regional_token::{
    EphemSymbolModifierRegionalTokens, IdentRegionalToken, RegionalTokenIdx,
};
use husky_syn_expr::SynPatternData;
use husky_token_data::LiteralTokenData;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SemaPatternData {
    /// example: `1`
    Literal {
        regional_token_idx: RegionalTokenIdx,
        literal: LiteralTokenData,
    },
    /// example: `a`
    Ident {
        symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaPatternEntry {
    data_result: SemaExprDataResult<SemaPatternData>,
    ty_result: SemaExprTypeResult<FlyTerm>,
}
