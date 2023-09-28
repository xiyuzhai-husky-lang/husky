use husky_sema_expr::SemaHtmlArgumentExpr;
use husky_syn_expr::SynHtmlArgumentExpr;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerHtmlArgumentExpr {
    property_ident: Ident,
    expr: HirEagerExprIdx,
}

impl vec_like::AsVecMapEntry for HirEagerHtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.property_ident
    }

    fn key_ref(&self) -> &Self::K {
        &self.property_ident
    }
}

impl ToHirEager for SemaHtmlArgumentExpr {
    type Output = HirEagerHtmlArgumentExpr;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            SemaHtmlArgumentExpr::Expanded {
                property_ident,
                expr,
                ..
            } => HirEagerHtmlArgumentExpr {
                property_ident: property_ident.ident(),
                expr: expr.to_hir_eager(builder),
            },
            SemaHtmlArgumentExpr::Shortened {
                lcurl,
                property_ident,
                rcurl,
            } => HirEagerHtmlArgumentExpr {
                property_ident: property_ident.ident(),
                expr: todo!(),
            },
        }
    }
}
