use super::*;

impl TranspileToRust for TraitItemHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            TraitItemHirDefn::AssociatedFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::AssociatedType(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::AssociatedVal(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRust for TraitAssociatedFnHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TraitMethodFnHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TraitAssociatedTypeHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TraitAssociatedValHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}
