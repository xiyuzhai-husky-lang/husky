use super::*;
use husky_hir_decl::TraitForTypeImplBlockHirDecl;

impl TranspileToRust for TraitForTypeImplBlockHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TraitForTypeImplBlockHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::Impl)
    }
}
