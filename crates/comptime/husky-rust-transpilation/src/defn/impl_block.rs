mod trai_for_ty_impl_block;
mod ty_impl_block;

use super::*;

impl TranspileToRustWith for ImplBlockHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.transpile_to_rust(builder),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}
