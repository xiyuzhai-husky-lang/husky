use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Expr(Arc<FeatureExpr>),
    Block(Arc<FeatureBlock>),
}

impl FeatureRepr {
    pub fn feature(&self) -> FeaturePtr {
        match self {
            FeatureRepr::Expr(expr) => expr.feature,
            FeatureRepr::Block(block) => block.feature,
        }
    }
}

impl From<Arc<FeatureExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureExpr>) -> Self {
        Self::Expr(expr)
    }
}

impl From<Arc<FeatureBlock>> for FeatureRepr {
    fn from(block: Arc<FeatureBlock>) -> Self {
        Self::Block(block)
    }
}
