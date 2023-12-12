use super::*;
use either::*;
use husky_entity_path::PreludeTraitPath;
use husky_entity_syn_tree::HasAssociatedItemPaths;
use husky_hir_decl::TraitForTypeImplBlockHirDecl;

impl TranspileToRustWith for TraitForTypeImplBlockHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl();
        let path = hir_decl.path(db);
        match path.trai_path(db).refine(db) {
            // skipping visualize, will be replaced by cfg to achieve the same skipping
            Left(PreludeTraitPath::VISUALIZE | PreludeTraitPath::INT_INDEX) => return,
            _ => (),
        }
        builder.rustfmt_skip();
        hir_decl.transpile_to_rust(builder);
        builder.curly_block(|builder| {
            for &(_, trai_for_ty_item_path) in hir_decl.path(db).associated_item_paths(db) {
                if let Some(hir_defn) = trai_for_ty_item_path.hir_defn(db) {
                    builder.on_fresh_paragraph(|builder| hir_defn.transpile_to_rust(builder));
                }
            }
        })
    }
}

impl TranspileToRustWith for TraitForTypeImplBlockHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.with_hir_eager_expr_region(self.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Impl);
            self.template_parameters(db).transpile_to_rust(builder);
            self.trai(db).transpile_to_rust(builder);
            builder.keyword(RustKeyword::ConnectionFor);
            self.self_ty(db).transpile_to_rust(builder)
        })
    }
}
