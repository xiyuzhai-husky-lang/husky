use crate::{db::HirExprDb, *};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_hir_eager_expr::builder::HirEagerExprBuilder;
use husky_hir_lazy_expr::builder::HirLazyExprBuilder;
use husky_syn_expr::{SynExpr, SynExprRegion};

pub enum HirExprBuilder<'a> {
    Eager(HirEagerExprBuilder<'a>),
    Lazy(HirLazyExprBuilder<'a>),
}

impl<'a> HirExprBuilder<'a> {
    pub fn new(db: &'a dyn HirExprDb, expr_region: SynExprRegion) -> Self {
        match expr_region_contains_gn(db, expr_region) {
            true => todo!(),
            false => HirExprBuilder::Eager(HirEagerExprBuilder::new(db)),
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
    let expr_ty_region = db.expr_ty_region(syn_expr_region);
    for (expr_idx, expr) in syn_expr_region_data.expr_arena().indexed_iter() {
        match expr {
            SynExpr::Literal(_, _) => (),
            SynExpr::PrincipalEntityPath {
                opt_path: Some(PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path))),
                ..
            } => todo!(),
            SynExpr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall {
                function,
                generic_arguments,
                lpar_token_idx,
                items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Ritchie {
                ritchie_kind_token_idx,
                ritchie_kind,
                lpar_token,
                parameter_ty_items,
                rpar_token_idx,
                light_arrow_token,
                return_ty_expr,
            } => todo!(),
            SynExpr::FunctionCall {
                function,
                generic_arguments,
                lpar_token_idx,
                items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            SynExpr::MethodApplicationOrCall {
                self_argument,
                dot_token_idx,
                ident_token,
                generic_arguments,
                lpar_token_idx,
                items,
                rpar_token_idx,
            } => todo!(),
            _ => (),
        }
    }
    false
}
