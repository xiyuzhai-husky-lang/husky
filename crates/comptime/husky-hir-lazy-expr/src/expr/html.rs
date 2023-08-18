use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HirLazyHtmlArgumentExpr {
    Expanded {
        property_ident: Ident,
        expr: HirLazyExprIdx,
    },
    Shortened {
        property_ident: Ident,
    },
}

impl vec_like::AsVecMapEntry for HirLazyHtmlArgumentExpr {
    type K = Ident;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            HirLazyHtmlArgumentExpr::Expanded { property_ident, .. }
            | HirLazyHtmlArgumentExpr::Shortened { property_ident, .. } => *property_ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            HirLazyHtmlArgumentExpr::Expanded { property_ident, .. }
            | HirLazyHtmlArgumentExpr::Shortened { property_ident, .. } => property_ident,
        }
    }
}
