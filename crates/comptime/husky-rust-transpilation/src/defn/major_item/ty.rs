use crate::{defn::attr::Attrs, expr::site::HirEagerExprSite};

use super::*;

use husky_coword::Ident;
use husky_entity_syn_tree::{HasAttrPaths, HasTypeVariantPaths};
use husky_hir_decl::{
    HasHirDecl, PropsFieldHirInitialization, PropsStructFieldHirDecl, TupleFieldHirDecl,
    TypeVariantHirDecl,
};
use husky_hir_ty::HirType;

impl TranspileToRustWith for TypeHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let rust_attrs = Attrs::new(self.path(db).attr_paths(db), builder);
        builder.value_conversion();
        rust_attrs.transpile_to_rust(builder);
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
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Enum);
            hir_decl.path(db).ident(db).transpile_to_rust(builder);
            hir_decl.template_parameters(db).transpile_to_rust(builder);
        });
        builder.bracketed_multiline_comma_list(
            RustBracket::MultilineCurl,
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
        let fields = hir_decl.fields(db);
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Struct);
            hir_decl.path(db).ident(db).transpile_to_rust(builder);
            hir_decl.template_parameters(db).transpile_to_rust(builder);
            builder.bracketed_multiline_comma_list(RustBracket::MultilineCurl, fields);
            builder.on_fresh_paragraph(|builder| {
                builder.keyword(RustKeyword::Impl);
                hir_decl.path(db).ident(db).transpile_to_rust(builder);
                // constructor
                builder.curly_block(|builder| {
                    builder.on_fresh_line(|builder| {
                        builder.keyword(RustKeyword::Pub);
                        builder.keyword(RustKeyword::Fn);
                        builder.ty_constructor_ident();
                        builder.bracketed_comma_list(
                            RustBracket::Par,
                            fields
                                .iter()
                                .filter_map(|&field| match field.initialization {
                                    Some(PropsFieldHirInitialization::Bind { .. }) => None,
                                    _ => Some(HirEagerParenateParameterFromField {
                                        ident: field.ident(),
                                        ty: field.ty(),
                                    }),
                                }),
                        );
                        builder.punctuation(RustPunctuation::LightArrow);
                        builder.self_ty();
                        builder.curly_block(|builder| {
                            for field in fields {
                                if let Some(PropsFieldHirInitialization::Bind { value }) =
                                    field.initialization
                                {
                                    builder.on_fresh_semicolon_line(|builder| {
                                        builder.keyword(RustKeyword::Let);
                                        field.ident().transpile_to_rust(builder);
                                        builder.punctuation(RustPunctuation::Assign);
                                        (value, HirEagerExprSite::new_root(None))
                                            .transpile_to_rust(builder)
                                    })
                                }
                            }
                            builder.on_fresh_line(|builder| {
                                builder.self_ty();
                                builder.bracketed_multiline_comma_list(
                                    RustBracket::Curl,
                                    fields.iter().map(|field| field.ident()),
                                )
                            })
                        })
                    })
                })
            })
        })
    }
}

struct HirEagerParenateParameterFromField {
    ident: Ident,
    ty: HirType,
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerParenateParameterFromField {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.ident.transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::Colon);
        self.ty.transpile_to_rust(builder)
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
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
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
