use super::*;

use husky_entity_syn_tree::HasTypeVariantPaths;
use husky_hir_decl::{HasHirDecl, PropsStructFieldHirDecl, TupleFieldHirDecl, TypeVariantHirDecl};

impl TranspileToRustWith for TypeHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            TypeHirDefn::Enum(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeHirDefn::PropsStruct(hir_defn) => hir_defn.transpile_to_rust(builder),
            TypeHirDefn::TupleStruct(hir_defn) => hir_defn.transpile_to_rust(builder),
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

impl TranspileToRustWith for EnumHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.eager_head(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Enum);
            hir_decl.path(db).ident(db).transpile_to_rust(builder);
            hir_decl.template_parameters(db).transpile_to_rust(builder);
        });
        builder.bracketed_multiline_comma_list(
            RustBracket::CurlSpaced,
            hir_decl
                .path(db)
                .ty_variant_paths(db)
                .iter()
                .map(|(_, path)| path.hir_decl(db)),
        )
    }
}

impl TranspileToRustWith for TypeVariantHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            TypeVariantHirDecl::Props(hir_decl) => todo!(),
            TypeVariantHirDecl::Unit(hir_decl) => hir_decl.transpile_to_rust(builder),
            TypeVariantHirDecl::Tuple(hir_decl) => hir_decl.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith for PropsStructHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.eager_head(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Struct);
            hir_decl.path(db).ident(db).transpile_to_rust(builder);
            hir_decl.template_parameters(db).transpile_to_rust(builder);
            builder.bracketed_multiline_comma_list(RustBracket::CurlSpaced, hir_decl.fields(db))
        })
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for PropsStructFieldHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Pub);
        self.ident().transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::Colon);
        self.ty().transpile_to_rust(builder)
    }
}

impl TranspileToRustWith for TupleStructHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.eager_head(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.on_fresh_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Pub);
                builder.keyword(RustKeyword::Struct);
                hir_decl.path(db).ident(db).transpile_to_rust(builder);
                hir_decl.template_parameters(db).transpile_to_rust(builder);
                builder.bracketed_comma_list(RustBracket::Par, hir_decl.fields(db))
            })
        })
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for TupleFieldHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.ty().transpile_to_rust(builder)
    }
}
