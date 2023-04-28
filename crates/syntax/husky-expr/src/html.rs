use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HtmlArgumentExpr {
    Expanded {
        argument_ident: IdentToken,
        eq: EqToken,
        lcurl: LeftCurlyBraceToken,
        expr: ExprIdx,
        rcurl: RightCurlyBraceToken,
    },
    Shortened {
        lcurl: LeftCurlyBraceToken,
        argument_ident: IdentToken,
        rcurl: RightCurlyBraceToken,
    },
}

impl vec_like::AsVecMapEntry for HtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            HtmlArgumentExpr::Expanded { argument_ident, .. }
            | HtmlArgumentExpr::Shortened { argument_ident, .. } => argument_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            HtmlArgumentExpr::Expanded { argument_ident, .. }
            | HtmlArgumentExpr::Shortened { argument_ident, .. } => argument_ident.ident_ref(),
        }
    }
}
