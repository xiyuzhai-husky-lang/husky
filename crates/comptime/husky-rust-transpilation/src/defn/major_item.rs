pub(crate) mod form;
pub(crate) mod trai;
pub(crate) mod ty;

use super::*;
use husky_hir_defn::defn::major_item::MajorItemHirDefn;

impl TranspileToRustWith for MajorItemHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            MajorItemHirDefn::Type(ty_defn) => ty_defn.transpile_to_rust(builder),
            MajorItemHirDefn::Trait(trai_defn) => trai_defn.transpile_to_rust(builder),
            MajorItemHirDefn::Form(form_defn) => form_defn.transpile_to_rust(builder),
        }
    }
}
