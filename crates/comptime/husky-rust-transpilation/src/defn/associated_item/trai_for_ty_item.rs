use either::Either;
use husky_entity_path::PreludeTraitPath;
use husky_hir_decl::HasHirDecl;
use husky_hir_ty::{
    HirConstSymbol, HirConstSymbolIndex, HirTemplateSymbol, HirTemplateSymbolClass,
};
use salsa::DebugWithDb;
use smallvec::SmallVec;

use super::*;
use crate::builder::keyword::RustKeyword;

impl TranspileToRustWith for TraitForTypeItemHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
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

impl TranspileToRustWith for TraitForTypeAssociatedFnHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        builder.keyword(RustKeyword::Fn);
        let path = self.path(db);
        path.ident(db).transpile_to_rust(builder);
        let hir_decl = self.hir_decl(db);
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
            hir_decl.template_parameters(db).transpile_to_rust(builder);
            let impl_block_path = path.impl_block(db);
            match impl_block_path.trai_path(db).refine(db) {
                Either::Left(PreludeTraitPath::UNVEIL) => {
                    builder.bracketed_list_with(RustBracket::Par, |builder| {
                        builder.heterogeneous_comma_list_items(
                            hir_decl.parenate_parameters(db).iter(),
                        );
                        builder.heterogeneous_comma_list_item_with(|builder| {
                            let hir_decl = impl_block_path.hir_decl(db).unwrap();
                            let runtime_const_symbols: SmallVec<[HirConstSymbol; 4]> = hir_decl
                                .template_parameters(db)
                                .iter()
                                .filter_map(|param| match param.symbol() {
                                    HirTemplateSymbol::Const(symbol) => (symbol.index(db).class()
                                        == HirTemplateSymbolClass::Runtime)
                                        .then_some(symbol),
                                    _ => None,
                                })
                                .collect();
                            builder.with_hir_eager_expr_region(
                                hir_decl.hir_eager_expr_region(db),
                                |builder| {
                                    builder.bracketed_comma_list(
                                        RustBracket::Par,
                                        runtime_const_symbols.iter().copied(),
                                    );
                                    builder.punctuation(RustPunctuation::Colon);
                                    builder.punctuation(RustPunctuation::Ambersand);
                                    builder.bracketed_comma_list(
                                        RustBracket::Par,
                                        runtime_const_symbols.iter().map(|symbol| symbol.ty(db)),
                                    )
                                },
                            )
                        })
                    })
                }
                _ => hir_decl.parenate_parameters(db).transpile_to_rust(builder),
            }
            builder.return_ty(hir_decl.return_ty(db))
        });
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRustWith for TraitForTypeMethodFnHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        builder.keyword(RustKeyword::Fn);
        self.path(db).ident(db).transpile_to_rust(builder);
        let hir_decl = self.hir_decl(db);
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
            hir_decl.template_parameters(db).transpile_to_rust(builder);
            builder.bracketed_list_with(RustBracket::Par, |builder| {
                builder.heterogeneous_comma_list_item(hir_decl.self_value_parameter(db));
                builder.heterogeneous_comma_list_items(hir_decl.parenate_parameters(db).iter())
            });
            builder.return_ty(hir_decl.return_ty(db))
        });
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRustWith for TraitForTypeAssociatedTypeHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
            builder.on_fresh_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Type);
                self.path(db).ident(db).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Assign);
                hir_decl.ty(db).transpile_to_rust(builder)
            })
        })
    }
}

impl TranspileToRustWith for TraitForTypeAssociatedValHirDefn {
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
