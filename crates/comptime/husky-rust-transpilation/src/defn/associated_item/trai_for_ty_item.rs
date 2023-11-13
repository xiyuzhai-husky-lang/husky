use super::*;
use crate::{builder::keyword::RustKeyword};

impl TranspileToRust for TraitForTypeItemHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => {
                hir_defn.transpile_to_rust(builder)
            }
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRust for TraitForTypeAssociatedFnHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for TraitForTypeMethodFnHirDefn {
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
        builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
            builder.heterogeneous_comma_list_item(hir_decl.self_value_parameter(db));
            builder.heterogeneous_comma_list_items(hir_decl.parenate_parameters(db).iter())
        });
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRust for TraitForTypeAssociatedTypeHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.on_new_semicolon_line(|builder| {
            builder.keyword(RustKeyword::Type);
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Eq);
            self.hir_decl(db)
                .associated_ty(db)
                .transpile_to_rust(builder)
        })
    }
}

impl TranspileToRust for TraitForTypeAssociatedValHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}
