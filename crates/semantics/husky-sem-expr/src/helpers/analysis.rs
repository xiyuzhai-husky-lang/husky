use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::region::RegionPath;
use husky_fly_term::dispatch::StaticDispatch;

use super::*;

#[salsa::tracked(jar = SemaExprJar)]
pub fn sem_expr_region_requires_lazy(db: &::salsa::Db, sem_expr_region: SemaExprRegion) -> bool {
    let sem_expr_region_data = sem_expr_region.data(db);
    match sem_expr_region_data.path() {
        RegionPath::Snippet(_) => return false, // ad hoc
        RegionPath::Decl(path) => match path {
            ItemPath::MajorItem(MajorItemPath::Form(path)) => match path.major_form_kind(db) {
                MajorFormKind::Ritchie(ritchie_item_kind) => match ritchie_item_kind {
                    RitchieItemKind::Fn => todo!(),
                    RitchieItemKind::Gn => todo!(),
                    RitchieItemKind::Vn => todo!(),
                    RitchieItemKind::Pn => todo!(),
                    RitchieItemKind::Qn => todo!(),
                    RitchieItemKind::Tn => todo!(),
                },
                _ => return false,
            },
            ItemPath::AssocItem(path) => todo!(),
            _ => return false,
        },
        RegionPath::Defn(path) => match path {
            ItemPath::MajorItem(MajorItemPath::Form(path)) => match path.major_form_kind(db) {
                MajorFormKind::Ritchie(_) => todo!(),
                MajorFormKind::TypeAlias => todo!(),
                MajorFormKind::Val => (),
                MajorFormKind::Formal => todo!(),
                MajorFormKind::Const => todo!(),
            },
            ItemPath::AssocItem(path) => todo!(),
            _ => return false,
        },
    }
    for sem_expr_entry in sem_expr_region_data.sem_expr_arena().iter() {
        match sem_expr_entry.data() {
            SemaExprData::PrincipalEntityPath {
                path: PrincipalEntityPath::MajorItem(MajorItemPath::Form(path)),
                ..
            } => match path.major_form_kind(db) {
                MajorFormKind::Ritchie(ritchie_item_kind) if ritchie_item_kind.is_lazy() => {
                    return true
                }
                _ => (),
            },
            SemaExprData::MajorItemPathAssocItem {
                static_dispatch: StaticDispatch::AssocGn,
                ..
            }
            | SemaExprData::MethodGnCall { .. } => return true,
            _ => (),
        }
    }
    false
}
