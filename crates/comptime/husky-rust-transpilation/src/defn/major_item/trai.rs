use husky_token_data::Keyword;

use super::*;

impl TranspileToRust for TraitHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        Keyword::Pub.transpile_to_rust(builder);
        let hir_decl = self.hir_decl(db);
        Keyword::Trait.transpile_to_rust(builder);
        hir_decl.path(db).ident(db).transpile_to_rust(builder);
        hir_decl.template_parameters(db);
        todo!()
    }
}
