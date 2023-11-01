use super::*;
use crate::builder::keyword::RustKeyword;
use husky_entity_syn_tree::HasAssociatedItemPaths;

impl TranspileToRust for TraitHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.keyword(RustKeyword::Pub);
        let hir_decl = self.hir_decl(db);
        builder.keyword(RustKeyword::Trait);
        hir_decl.path(db).ident(db).transpile_to_rust(builder);
        hir_decl.template_parameters(db).transpile_to_rust(builder);
        builder.curly_block(|builder| {
            for &(_, trai_item_path) in self.path(db).associated_item_paths(db) {
                if let Some(trai_item_hir_defn) = trai_item_path.hir_defn(db) {
                    trai_item_hir_defn.transpile_to_rust(builder)
                }
            }
        })
    }
}
