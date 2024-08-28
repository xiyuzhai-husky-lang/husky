use super::*;
use husky_regional_token::{EqRegionalToken, InlineLcurlRegionalToken, InlineRcurlRegionalToken};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemaHtmxArgumentExpr {
    Expanded {
        property_ident: IdentRegionalToken,
        eq: EqRegionalToken,
        lcurl: InlineLcurlRegionalToken,
        argument: SemExprIdx,
        rcurl: InlineRcurlRegionalToken,
    },
    Shortened {
        lcurl: InlineLcurlRegionalToken,
        property_ident: IdentRegionalToken,
        // todo: argument: SemExprIdx,
        rcurl: InlineRcurlRegionalToken,
    },
}

impl SemaHtmxArgumentExpr {
    pub fn expr(self) -> SemExprIdx {
        match self {
            SemaHtmxArgumentExpr::Expanded { argument, .. } => argument,
            SemaHtmxArgumentExpr::Shortened { .. } => todo!(),
        }
    }
}

impl vec_like::AsVecMapEntry for SemaHtmxArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            SemaHtmxArgumentExpr::Expanded { property_ident, .. }
            | SemaHtmxArgumentExpr::Shortened { property_ident, .. } => property_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            SemaHtmxArgumentExpr::Expanded {
                property_ident: argument_ident,
                ..
            }
            | SemaHtmxArgumentExpr::Shortened {
                property_ident: argument_ident,
                ..
            } => argument_ident.ident_ref(),
        }
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_sem_html_argument_expr(
        &mut self,
        expr: SynHtmlArgumentExpr,
    ) -> SemaHtmxArgumentExpr {
        match expr {
            SynHtmlArgumentExpr::Expanded {
                property_ident,
                eq,
                lcurl,
                expr,
                rcurl,
            } => SemaHtmxArgumentExpr::Expanded {
                property_ident,
                eq,
                lcurl,
                argument: self.build_expr(expr, ExpectAnyOriginal),
                rcurl,
            },
            SynHtmlArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            } => SemaHtmxArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            },
        }
    }
}
