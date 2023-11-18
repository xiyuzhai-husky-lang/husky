use super::*;
use crate::builder::keyword::RustKeyword;
use husky_entity_syn_tree::HasAssociatedItemPaths;
use husky_hir_decl::TraitHirDecl;

impl TranspileToRust for TraitHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        self.hir_decl(db).transpile_to_rust(builder);
        builder.curly_block(|builder| {
            for &(_, trai_item_path) in self.path(db).associated_item_paths(db) {
                if let Some(trai_item_hir_defn) = trai_item_path.hir_defn(db) {
                    trai_item_hir_defn.transpile_to_rust(builder)
                }
            }
        })
    }
}

impl TranspileToRust for TraitHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db();
        builder.eager_head(self.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Trait);
            self.path(db).ident(db).transpile_to_rust(builder);
            self.template_parameters(db).transpile_to_rust(builder);
        })
    }
}
