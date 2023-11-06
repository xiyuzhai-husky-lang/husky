use husky_hir_decl::TypeMethodFnHirDecl;

use super::*;

impl TranspileToRust for TypeItemHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            TypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRust for TypeAssociatedFnHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        builder.keyword(RustKeyword::Fn);
        self.path(db).ident(db).transpile_to_rust(builder);
        let hir_decl = self.hir_decl(db);
        hir_decl.template_parameters(db).transpile_to_rust(builder);
        hir_decl.parenate_parameters(db).transpile_to_rust(builder);
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRust for TypeMethodFnHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        self.hir_decl(db).transpile_to_rust(builder);
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRust for TypeMethodFnHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_eager_expr_region = self.hir_eager_expr_region(db);
        builder.eager_head(hir_eager_expr_region, |builder| {
            builder.keyword(RustKeyword::Fn);
            self.path(db).ident(db).transpile_to_rust(builder);
            self.template_parameters(db).transpile_to_rust(builder);
            builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                builder.heterogeneous_comma_list_item(self.self_value_parameter(db));
                builder.heterogeneous_comma_list_items(self.parenate_parameters(db).iter())
            })
        })
    }
}

impl TranspileToRust for TypeAssociatedTypeHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TypeAssociatedValHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TypeMemoizedFieldHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        builder.keyword(RustKeyword::Fn);
        self.path(db).ident(db).transpile_to_rust(builder);
        let hir_decl = self.hir_decl(db);
        builder
            .heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| builder.self_value());
        builder.eager_body(hir_eager_expr_region, body)
    }
}
