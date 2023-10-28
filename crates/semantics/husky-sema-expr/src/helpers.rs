use crate::*;

#[salsa::tracked(jar = SemaExprJar)]
pub fn syn_expr_region_contains_gn(db: &dyn SemaExprDb, syn_expr_region: SynExprRegion) -> bool {
    let syn_expr_region_data = syn_expr_region.data(db);
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    for sema_expr_entry in sema_expr_region.sema_expr_arena_ref().iter() {
        match sema_expr_entry.data() {
            SemaExprData::PrincipalEntityPath {
                path: PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)),
                ..
            } => match path.fugitive_kind(db) {
                FugitiveKind::FunctionGn => return true,
                FugitiveKind::FunctionFn | FugitiveKind::AliasType | FugitiveKind::Val => (),
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
