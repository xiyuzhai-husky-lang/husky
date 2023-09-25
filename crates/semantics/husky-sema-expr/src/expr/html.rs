use husky_regional_token::{LcurlRegionalToken, RcurlRegionalToken, RegionalEqToken};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemaHtmlArgumentExpr {
    Expanded {
        property_ident: IdentRegionalToken,
        eq: RegionalEqToken,
        lcurl: LcurlRegionalToken,
        expr: SemaExprIdx,
        rcurl: RcurlRegionalToken,
    },
    Shortened {
        lcurl: LcurlRegionalToken,
        property_ident: IdentRegionalToken,
        rcurl: RcurlRegionalToken,
    },
}

impl vec_like::AsVecMapEntry for SemaHtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            SemaHtmlArgumentExpr::Expanded { property_ident, .. }
            | SemaHtmlArgumentExpr::Shortened { property_ident, .. } => property_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            SemaHtmlArgumentExpr::Expanded {
                property_ident: argument_ident,
                ..
            }
            | SemaHtmlArgumentExpr::Shortened {
                property_ident: argument_ident,
                ..
            } => argument_ident.ident_ref(),
        }
    }
}
