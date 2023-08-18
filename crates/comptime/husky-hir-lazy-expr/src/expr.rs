use crate::*;
use husky_syn_expr::SynExprIdx;

pub type HirLazyExprArena = Arena<HirLazyExpr>;
pub type HirLazyExprIdx = ArenaIdx<HirLazyExpr>;
pub type HirLazyExprIdxRange = ArenaIdxRange<HirLazyExpr>;
pub type HirLazyExprMap<V> = ArenaMap<HirLazyExpr, V>;

#[derive(Debug, PartialEq, Eq)]
pub enum HirLazyExpr {}

impl ToHirLazy for SynExprIdx {
    type Output = HirLazyExprIdx;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        todo!()
    }
}
