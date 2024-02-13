use husky_regional_token::{EqRegionalToken, InlineLcurlRegionalToken, InlineRcurlRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemaHtmlArgumentExpr {
    Expanded {
        property_ident: IdentRegionalToken,
        eq: EqRegionalToken,
        lcurl: InlineLcurlRegionalToken,
        argument: SemaExprIdx,
        rcurl: InlineRcurlRegionalToken,
    },
    Shortened {
        lcurl: InlineLcurlRegionalToken,
        property_ident: IdentRegionalToken,
        // todo: argument: SemaExprIdx,
        rcurl: InlineRcurlRegionalToken,
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

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_html_argument_expr(
        &mut self,
        expr: SynHtmlArgumentExpr,
    ) -> SemaHtmlArgumentExpr {
        match expr {
            SynHtmlArgumentExpr::Expanded {
                property_ident,
                eq,
                lcurl,
                expr,
                rcurl,
            } => SemaHtmlArgumentExpr::Expanded {
                property_ident,
                eq,
                lcurl,
                argument: self.build_sema_expr(expr, ExpectAnyOriginal),
                rcurl,
            },
            SynHtmlArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            } => SemaHtmlArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            },
        }
    }
}
