use husky_sem_expr::SemaHtmxArgumentExpr;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerHtmlArgumentExpr {
    property_ident: Ident,
    expr: HirEagerExprIdx,
}

impl HirEagerHtmlArgumentExpr {
    pub fn property_ident(&self) -> Ident {
        self.property_ident
    }

    pub fn expr(&self) -> HirEagerExprIdx {
        self.expr
    }
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

impl ToHirEager for SemaHtmxArgumentExpr {
    type Output = HirEagerHtmlArgumentExpr;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            SemaHtmxArgumentExpr::Expanded {
                property_ident,
                argument: expr,
                ..
            } => HirEagerHtmlArgumentExpr {
                property_ident: property_ident.ident(),
                expr: expr.to_hir_eager(builder),
            },
            SemaHtmxArgumentExpr::Shortened { property_ident, .. } => HirEagerHtmlArgumentExpr {
                property_ident: property_ident.ident(),
                expr: todo!(),
            },
        }
    }
}
