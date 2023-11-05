use super::*;
use husky_entity_syn_tree::HasAssociatedItemPaths;
use husky_hir_decl::TraitForTypeImplBlockHirDecl;

impl TranspileToRust for TraitForTypeImplBlockHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl();
        hir_decl.transpile_to_rust(builder);
        builder.curly_block(|builder| {
            for &(_, trai_for_ty_item_path) in hir_decl.path(db).associated_item_paths(db) {
                if let Some(hir_defn) = trai_for_ty_item_path.hir_defn(db) {
                    builder.make_defn_fresh_lines();
                    hir_defn.transpile_to_rust(builder);
                }
            }
        })
    }
}

impl TranspileToRust for TraitForTypeImplBlockHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::Impl)
    }
}
