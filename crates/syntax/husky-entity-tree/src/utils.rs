use crate::*;
use smallvec::{smallvec, SmallVec};
use vec_like::SmallVecPairMap;

pub trait HasTypeSideTraitForTypeImplBlockPathsMap: Copy {
    fn ty_side_trai_for_ty_impl_block_paths_map<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> &'a SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2>;
}

impl HasTypeSideTraitForTypeImplBlockPathsMap for TypePath {
    fn ty_side_trai_for_ty_impl_block_paths_map<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> &'a SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2> {
        ty_side_trai_for_ty_impl_block_paths_map(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn ty_side_trai_for_ty_impl_block_paths_map(
    db: &dyn EntityTreeDb,
    ty_path: TypePath,
) -> SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2> {
    let crate_path = ty_path.crate_path(db);
    let bundle = db.entity_tree_bundle(crate_path).expect("should be valid");
    let mut map: SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2> =
        Default::default();
    for path in bundle.trai_for_ty_impl_block_paths_filtered_by_ty_path(db, ty_path) {
        map.update_value_or_insert(path.trai_path(db), |v| v.push(path), smallvec![path])
    }
    map
}
