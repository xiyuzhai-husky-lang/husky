use super::*;
use husky_regional_token::{EqRegionalToken, InlineLcurlRegionalToken, InlineRcurlRegionalToken};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemHtmxArgumentExpr {
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

impl SemHtmxArgumentExpr {
    pub fn expr(self) -> SemExprIdx {
        match self {
            SemHtmxArgumentExpr::Expanded { argument, .. } => argument,
            SemHtmxArgumentExpr::Shortened { .. } => todo!(),
        }
    }
}

impl vec_like::AsVecMapEntry for SemHtmxArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            SemHtmxArgumentExpr::Expanded { property_ident, .. }
            | SemHtmxArgumentExpr::Shortened { property_ident, .. } => property_ident.ident(),
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            SemHtmxArgumentExpr::Expanded {
                property_ident: argument_ident,
                ..
            }
            | SemHtmxArgumentExpr::Shortened {
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
    ) -> SemHtmxArgumentExpr {
        match expr {
            SynHtmlArgumentExpr::Expanded {
                property_ident,
                eq,
                lcurl,
                expr,
                rcurl,
            } => SemHtmxArgumentExpr::Expanded {
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
            } => SemHtmxArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            },
        }
    }
}
