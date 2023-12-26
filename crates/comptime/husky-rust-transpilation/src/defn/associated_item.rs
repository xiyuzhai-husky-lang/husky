mod trai_for_ty_item;
mod trai_item;
mod ty_item;

use super::*;

impl TranspileToRustWith for AssociatedItemHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        todo!();
        match self {
            AssociatedItemHirDefn::TypeItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            AssociatedItemHirDefn::TraitItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            AssociatedItemHirDefn::TraitForTypeItem(hir_defn) => {
                hir_defn.transpile_to_rust(builder)
            }
        }
    }
}
