use crate::{db::HirExprDb, *};
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_entity_taxonomy::FugitiveKind;
use husky_expr_ty::{MethodCallOrApplicationDisambiguation, SynExprDisambiguation};
use husky_fluffy_term::MethodFluffySignature;
use husky_hir_eager_expr::builder::HirEagerExprBuilder;
use husky_hir_lazy_expr::builder::HirLazyExprBuilder;
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
            SynExprData::Literal(_, _) => (),
            SynExprData::PrincipalEntityPath {
                opt_path: Some(PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path))),
                ..
            } => match path.fugitive_kind(db) {
                FugitiveKind::Gn => return true,
                FugitiveKind::Fn | FugitiveKind::AliasType | FugitiveKind::Val => (),
            },
            SynExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExprData::MethodApplicationOrCall {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                template_arguments,
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
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
