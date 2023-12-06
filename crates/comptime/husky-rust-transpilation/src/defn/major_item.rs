pub(crate) mod fugitive;
pub(crate) mod trai;
pub(crate) mod ty;

use super::*;

impl TranspileToRustWith for MajorItemHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            MajorItemHirDefn::Type(ty_defn) => ty_defn.transpile_to_rust(builder),
            MajorItemHirDefn::Trait(trai_defn) => trai_defn.transpile_to_rust(builder),
            MajorItemHirDefn::Fugitive(fugitive_defn) => fugitive_defn.transpile_to_rust(builder),
        }
    }
}
