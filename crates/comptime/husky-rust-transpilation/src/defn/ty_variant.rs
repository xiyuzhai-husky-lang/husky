use super::*;
use husky_hir_decl::{EnumTupleVariantField, EnumTupleVariantHirDecl, EnumUnitTypeVariantHirDecl};

impl TranspileToRust for TypeVariantHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRust for EnumUnitTypeVariantHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        self.path(db).ident(db).transpile_to_rust(builder)
    }
}

impl TranspileToRust for EnumTupleVariantHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.eager_head(self.hir_eager_expr_region(db), |builder| {
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Curl, self.fields(db))
        })
    }
}

impl TranspileToRust<HirEagerExprRegion> for EnumTupleVariantField {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.ty().transpile_to_rust(builder)
    }
}
