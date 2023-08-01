use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HtmlArgumentExpr {
    Expanded {
        property_ident: IdentToken,
        eq: EqToken,
        lcurl: LcurlToken,
        expr: SynExprIdx,
        rcurl: RcurlToken,
    },
    Shortened {
        lcurl: LcurlToken,
        property_ident: IdentToken,
        rcurl: RcurlToken,
    },
}

impl vec_like::AsVecMapEntry for HtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            HtmlArgumentExpr::Expanded { property_ident, .. }
            | HtmlArgumentExpr::Shortened { property_ident, .. } => property_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            HtmlArgumentExpr::Expanded {
                property_ident: argument_ident,
                ..
            }
            | HtmlArgumentExpr::Shortened {
                property_ident: argument_ident,
                ..
            } => argument_ident.ident_ref(),
        }
    }
}
