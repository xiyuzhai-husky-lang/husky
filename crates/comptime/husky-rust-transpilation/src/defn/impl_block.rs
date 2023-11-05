mod trai_for_ty_impl_block;
mod ty_impl_block;

use husky_hir_decl::ImplBlockHirDecl;

use super::*;

impl TranspileToRust for ImplBlockHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.hir_decl() {
            ImplBlockHirDecl::Type(hir_decl) => hir_decl.transpile_to_rust(builder),
            ImplBlockHirDecl::TraitForType(hir_decl) => hir_decl.transpile_to_rust(builder),
        }
    }
}
