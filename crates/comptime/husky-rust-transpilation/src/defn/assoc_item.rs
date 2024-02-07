mod trai_for_ty_item;
mod trai_item;
mod ty_item;

use super::*;

impl TranspileToRustWith for AssocItemHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            AssocItemHirDefn::TypeItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            AssocItemHirDefn::TraitItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            AssocItemHirDefn::TraitForTypeItem(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}
