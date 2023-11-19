use husky_hir_decl::{FunctionFnFugitiveHirDecl, ValFugitiveHirDecl};
use husky_hir_expr::{HirExprIdx, HirExprRegion};

use super::*;
use crate::builder::keyword::RustKeyword;

impl TranspileToRust for FugitiveHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.transpile_to_rust(builder),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRust for FunctionFnFugitiveHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, body_hir_eager_expr_region)) =
            self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        self.hir_decl(db).transpile_to_rust(builder);
        builder.eager_body(body_hir_eager_expr_region, body)
    }
}

impl TranspileToRust for FunctionFnFugitiveHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.eager_head(self.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            self.path(db).ident(db).transpile_to_rust(builder);
            self.template_parameters(db).transpile_to_rust(builder);
            self.parenate_parameters(db).transpile_to_rust(builder);
            builder.return_ty(self.return_ty(db))
        })
    }
}

impl TranspileToRust for FunctionGnHirDefn {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder) {}
}

impl TranspileToRust for ValFugitiveHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((HirExprIdx::Eager(body), HirExprRegion::Eager(hir_eager_expr_region))) =
            self.body_with_hir_expr_region(db)
        else {
            return;
        };
        self.hir_decl(db).transpile_to_rust(builder);
        builder.eager_body(hir_eager_expr_region, body)
    }
}

impl TranspileToRust for ValFugitiveHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let hir_eager_expr_region = self.hir_eager_expr_region(builder.db());
        builder.eager_head(hir_eager_expr_region, |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            let db = builder.db();
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.bracketed_list_with(RustBracket::Par, |_| ());
            builder.return_ty(self.return_ty(db))
        })
    }
}
