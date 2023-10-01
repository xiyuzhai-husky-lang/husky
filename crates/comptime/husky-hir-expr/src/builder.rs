use crate::{db::HirExprDb, *};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_fluffy_term::{MethodFluffySignature, StaticDispatch};
use husky_hir_eager_expr::builder::HirEagerExprBuilder;
use husky_hir_lazy_expr::builder::HirLazyExprBuilder;
use husky_sema_expr::SemaExprData;
use husky_syn_expr::{SynExprData, SynExprRegion};

pub enum HirExprBuilder<'a> {
    Eager(HirEagerExprBuilder<'a>),
    Lazy(HirLazyExprBuilder<'a>),
}

impl<'a> HirExprBuilder<'a> {
    pub fn new(db: &'a dyn HirExprDb, syn_expr_region: SynExprRegion) -> Self {
        match expr_region_contains_gn(db, syn_expr_region) {
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

#[salsa::tracked(jar = HirExprJar)]
fn expr_region_contains_gn(db: &dyn HirExprDb, syn_expr_region: SynExprRegion) -> bool {
    let syn_expr_region_data = syn_expr_region.data(db);
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    for sema_expr_entry in sema_expr_region.sema_expr_arena_ref().iter() {
        match sema_expr_entry.data() {
            SemaExprData::PrincipalEntityPath {
                path: PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)),
                ..
            } => match path.fugitive_kind(db) {
                FugitiveKind::Gn => return true,
                FugitiveKind::Fn | FugitiveKind::AliasType | FugitiveKind::Val => (),
            },
            SemaExprData::AssociatedItem {
                static_dispatch: StaticDispatch::AssociatedGn,
                ..
            }
            | SemaExprData::GnCall { .. }
            | SemaExprData::MethodGnCall { .. } => return true,
            _ => (),
        }
    }
    false
}
