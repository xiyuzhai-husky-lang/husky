use husky_hir_decl::ValFugitiveHirDecl;
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

impl TranspileToRust for FunctionFnHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((body, hir_eager_expr_region)) = self.eager_body_with_hir_eager_expr_region(db)
        else {
            return;
        };
        builder.keyword(RustKeyword::Pub);
        let hir_decl = self.hir_decl(db);
        builder.keyword(RustKeyword::Fn);
        hir_decl.path(db).ident(db).transpile_to_rust(builder);
        hir_decl.template_parameters(db).transpile_to_rust(builder);
        hir_decl.parenate_parameters(db).transpile_to_rust(builder);
        builder.curly_block_with_hir_eager_expr_region(hir_eager_expr_region, |builder| {
            body.transpile_to_rust(builder)
        })
    }
}

impl TranspileToRust for FunctionGnHirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {}
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
        builder.curly_block_with_hir_eager_expr_region(hir_eager_expr_region, |builder| {
            body.transpile_to_rust(builder)
        })
    }
}

impl TranspileToRust for ValFugitiveHirDecl {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.keyword(RustKeyword::Fn);
        let db = builder.db();
        self.path(db).ident(db).transpile_to_rust(builder);
        builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |_| ())
        // ad hoc
    }
}
