use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::{
    path::{major_item::MajorItemPath, ItemPath, PrincipalEntityPath},
    region::RegionPath,
};
use husky_fly_term::dispatch::StaticDispatch;

use super::*;

#[salsa::tracked(jar = SemExprJar)]
pub fn sem_expr_region_requires_lazy(db: &::salsa::Db, sem_expr_region: SemExprRegion) -> bool {
    let sem_expr_region_data = sem_expr_region.data(db);
    match sem_expr_region_data.path() {
        RegionPath::CrateDecl(_) => todo!(),
        RegionPath::Script(_) => return false, // ad hoc
        RegionPath::ItemDecl(path) => match path {
            ItemPath::MajorItem(MajorItemPath::Form(path)) => match path.kind(db) {
                MajorFormKind::Ritchie(ritchie_item_kind) => match ritchie_item_kind {
                    RitchieItemKind::Fn => todo!(),
                    RitchieItemKind::Gn => todo!(),
                    RitchieItemKind::Vn => todo!(),
                    RitchieItemKind::Pn => todo!(),
                    RitchieItemKind::Qn => todo!(),
                    RitchieItemKind::Bn => todo!(),
                    RitchieItemKind::Sn => todo!(),
                    RitchieItemKind::Tn => todo!(),
                },
                _ => return false,
            },
            ItemPath::AssocItem(path) => todo!(),
            _ => return false,
        },
        RegionPath::ItemDefn(path) => match path {
            ItemPath::MajorItem(MajorItemPath::Form(path)) => match path.kind(db) {
                MajorFormKind::Ritchie(_) => todo!(),
                MajorFormKind::TypeAlias => todo!(),
                MajorFormKind::TypeVar => todo!(),
                MajorFormKind::Val => (),
                MajorFormKind::Conceptual => todo!(),
                MajorFormKind::Static => todo!(),
                MajorFormKind::Compterm => todo!(),
            },
            ItemPath::AssocItem(path) => todo!(),
            _ => return false,
        },
    }
    for sem_expr_entry in sem_expr_region_data.sem_expr_arena().iter() {
        match sem_expr_entry.data() {
            SemExprData::PrincipalEntityPath {
                path: PrincipalEntityPath::MajorItem(MajorItemPath::Form(path)),
                ..
            } => match path.kind(db) {
                MajorFormKind::Ritchie(ritchie_item_kind) if ritchie_item_kind.is_lazy() => {
                    return true
                }
                _ => (),
            },
            SemExprData::MajorItemPathAssocItem {
                static_dispatch: StaticDispatch::AssocGn,
                ..
            }
            | SemExprData::MethodGnCall { .. } => return true,
            _ => (),
        }
    }
    false
}
