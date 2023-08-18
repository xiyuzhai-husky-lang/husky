use crate::{db::HirExprDb, *};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_entity_taxonomy::FugitiveKind;
use husky_expr_ty::{MethodCallOrApplicationDisambiguation, SynExprDisambiguation};
use husky_fluffy_term::MethodFluffySignature;
use husky_hir_eager_expr::builder::HirEagerExprBuilder;
use husky_hir_lazy_expr::builder::HirLazyExprBuilder;
use husky_syn_expr::{SynExpr, SynExprRegion};

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
    for (syn_expr_idx, syn_expr) in syn_expr_region_data.expr_arena().indexed_iter() {
        match syn_expr {
            SynExpr::Literal(_, _) => (),
            SynExpr::PrincipalEntityPath {
                opt_path: Some(PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path))),
                ..
            } => match path.fugitive_kind(db) {
                FugitiveKind::Gn => return true,
                FugitiveKind::Fn | FugitiveKind::AliasType | FugitiveKind::Val => (),
            },
            SynExpr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
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
            } => {
                let SynExprDisambiguation::MethodCallOrApplication(disambiguation) =
                    expr_ty_region.expr_disambiguation_unwrapped(syn_expr_idx)
                else {
                    unreachable!()
                };
                match disambiguation {
                    MethodCallOrApplicationDisambiguation::MethodCall {
                        method_dispatch,
                        ritchie_parameter_argument_matches,
                    } => match method_dispatch.signature() {
                        MethodFluffySignature::MethodFn(_) => (),
                        MethodFluffySignature::MethodFunction(_) => (),
                        MethodFluffySignature::MethodGn => return true,
                    },
                }
            }
            _ => (),
        }
    }
    false
}
