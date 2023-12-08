use husky_hir_decl::{FunctionFnFugitiveHirDecl, ValFugitiveHirDecl};
use husky_hir_expr::{HirExprIdx, HirExprRegion};

use super::*;
use crate::builder::keyword::RustKeyword;

impl TranspileToRustWith for FugitiveHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            FugitiveHirDefn::FunctionFn(hir_defn) => hir_defn.transpile_to_rust(builder),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.transpile_to_rust(builder),
            FugitiveHirDefn::FunctionGn(hir_defn) => hir_defn.transpile_to_rust(builder),
            FugitiveHirDefn::TypeAlias(_) => todo!(),
        }
    }
}

impl TranspileToRustWith for FunctionFnHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
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

impl TranspileToRustWith for FunctionFnFugitiveHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.with_hir_eager_expr_region(self.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            self.path(db).ident(db).transpile_to_rust(builder);
            self.template_parameters(db).transpile_to_rust(builder);
            self.parenate_parameters(db).transpile_to_rust(builder);
            builder.return_ty(self.return_ty(db))
        })
    }
}

impl TranspileToRustWith for FunctionGnHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {}
}

impl TranspileToRustWith for ValHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        builder.val_item_attr(hir_decl.return_ty(db).always_copyable(db));
        hir_decl.transpile_to_rust(builder);
        match self.body_with_hir_expr_region(db) {
            Some((HirExprIdx::Eager(body), HirExprRegion::Eager(hir_eager_expr_region))) => {
                builder.eager_body(hir_eager_expr_region, body)
            }
            _ => builder.curly_block(|builder| builder.on_fresh_line(|builder| builder.todo())),
        }
    }
}

impl TranspileToRustWith for ValFugitiveHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let hir_eager_expr_region = self.hir_eager_expr_region(builder.db());
        builder.with_hir_eager_expr_region(hir_eager_expr_region, |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            let db = builder.db();
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.bracketed_list_with(RustBracket::Par, |_| ());
            builder.return_ty(self.return_ty(db))
        })
    }
}
