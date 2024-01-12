use crate::*;
use husky_entity_path::TypePath;
use husky_entity_syn_tree::HasTypeVariantPaths;

#[salsa::tracked(jar = HirDeclJar)]
pub fn enum_ty_has_only_unit_variants(db: &::salsa::Db, ty_path: TypePath) -> bool {
    for &(_, ty_variant_path) in ty_path.ty_variant_paths(db) {
        let TypeVariantHirDecl::Unit(_) = ty_variant_path.hir_decl(db).unwrap() else {
            return false;
        };
    }
    true
}
