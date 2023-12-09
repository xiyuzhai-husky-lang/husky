use super::*;

impl TranspileToRustWith for TraitItemHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            TraitItemHirDefn::AssociatedFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::MethodFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::AssociatedType(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitItemHirDefn::AssociatedVal(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith for TraitAssociatedFnHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRustWith for TraitMethodFnHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRustWith for TraitAssociatedTypeHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRustWith for TraitAssociatedValHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.val_item_attr(
            hir_decl.path(db).into(),
            todo!(),
            hir_decl.return_ty(db).always_copyable(db),
        );
        todo!()
    }
}
