use crate::{db::HirExprDb, *};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_fluffy_term::{MethodFluffySignature, StaticDispatch};
use husky_hir_eager_expr::builder::HirEagerExprBuilder;
use husky_hir_lazy_expr::builder::HirLazyExprBuilder;
use husky_sema_expr::{helpers::syn_expr_region_contains_gn, SemaExprData};
use husky_syn_expr::{SynExprData, SynExprRegion};

pub enum HirExprBuilder<'a> {
    Eager(HirEagerExprBuilder<'a>),
    Lazy(HirLazyExprBuilder<'a>),
}

impl<'a> HirExprBuilder<'a> {
    pub fn new(db: &'a dyn HirExprDb, syn_expr_region: SynExprRegion) -> Self {
        match syn_expr_region_contains_gn(db, syn_expr_region) {
            true => HirExprBuilder::Lazy(HirLazyExprBuilder::new(db, syn_expr_region)),
            false => HirExprBuilder::Eager(HirEagerExprBuilder::new(db, syn_expr_region)),
        }
    }

    pub fn build_hir_expr(&mut self, syn_expr_root: SynExprIdx) -> HirExprIdx {
        match self {
            HirExprBuilder::Eager(this) => this.build_hir_eager_expr(syn_expr_root).into(),
            HirExprBuilder::Lazy(this) => this.build_hir_lazy_expr(syn_expr_root).into(),
        }
    }

    pub fn finish(self) -> HirExprRegion {
        match self {
            HirExprBuilder::Eager(builder) => builder.finish().into(),
            HirExprBuilder::Lazy(builder) => builder.finish().into(),
        }
    }
}
