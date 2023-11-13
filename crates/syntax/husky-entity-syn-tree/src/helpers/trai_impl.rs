use super::*;

// trait side

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub fn trai_side_derive_any_trai_for_ty_impl_block_paths_map(
    db: &dyn EntitySynTreeDb,
    trai_path: TraitPath,
) -> SmallVec<[TraitForTypeImplBlockPath; 2]> {
    let crate_path = trai_path.crate_path(db);
    let bundle = db.item_syn_tree_bundle(crate_path);
    let mut paths: SmallVec<[TraitForTypeImplBlockPath; 2]> = smallvec![];
    for path in bundle.trai_for_ty_impl_block_paths_filtered_by_trai_path(db, trai_path) {
        match path.ty_sketch(db) {
            TypeSketch::DeriveAny => paths.push(path),
            TypeSketch::Path(_ty_path) => continue,
        }
    }
    paths
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub fn trai_side_path_leading_trai_for_ty_impl_block_paths_map(
    db: &dyn EntitySynTreeDb,
    trai_path: TraitPath,
) -> SmallVecPairMap<TypePath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 8> {
    let crate_path = trai_path.crate_path(db);
    let bundle = db.item_syn_tree_bundle(crate_path);
    let mut map: SmallVecPairMap<TypePath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 8> =
        Default::default();
    for path in bundle.trai_for_ty_impl_block_paths_filtered_by_trai_path(db, trai_path) {
        match path.ty_sketch(db) {
            TypeSketch::DeriveAny => continue,
            TypeSketch::Path(ty_path) => {
                if ty_path.crate_path(db) != crate_path {
                    map.update_value_or_insert(ty_path, |v| v.push(path), smallvec![path])
                }
            }
        }
    }
    map
}

// type side

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub fn ty_side_trai_for_ty_impl_block_paths_map(
    db: &dyn EntitySynTreeDb,
    ty_path: TypePath,
) -> SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2> {
    let crate_path = ty_path.crate_path(db);
    let bundle = db.item_syn_tree_bundle(crate_path);
    let mut map: SmallVecPairMap<TraitPath, SmallVec<[TraitForTypeImplBlockPath; 2]>, 2> =
        Default::default();
    for path in bundle.trai_for_ty_impl_block_paths_filtered_by_ty_path(db, ty_path) {
        map.update_value_or_insert(path.trai_path(db), |v| v.push(path), smallvec![path])
    }
    map
}
