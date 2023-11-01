use super::*;
use husky_entity_path::TypePath;

impl TranspileToRust for TypeHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            TypeHirDefn::Enum(_) => todo!(),
            TypeHirDefn::PropsStruct(_) => todo!(),
            TypeHirDefn::TupleStruct(_) => todo!(),
            TypeHirDefn::UnitStruct(_) => todo!(),
            TypeHirDefn::Extern(_) =>
            /* ad hoc */
            {
                ()
            }
            TypeHirDefn::Union(_) => todo!(),
        }
    }
}
