use super::*;
use crate::builder::keyword::RustKeyword;
use husky_hir_decl::decl::{MajorFunctionRitchieHirDecl, MajorValHirDecl};
use husky_hir_defn::defn::major_item::form::{
    function_ritchie::MajorFunctionRitchieHirDefn, val::MajorValHirDefn, MajorFormHirDefn,
};
use husky_hir_expr::{HirExprIdx, HirExprRegion};

impl TranspileToRustWith for MajorFormHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        match self {
            MajorFormHirDefn::Ritchie(hir_defn) => hir_defn.transpile_to_rust(builder),
            MajorFormHirDefn::Val(hir_defn) => hir_defn.transpile_to_rust(builder),
            MajorFormHirDefn::TypeAlias(_) => todo!(),
            MajorFormHirDefn::Const(_) => todo!(),
            MajorFormHirDefn::Static(_) => todo!(),
        }
    }
}

impl TranspileToRustWith for MajorFunctionRitchieHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let Some((HirExprIdx::Eager(body), HirExprRegion::Eager(body_hir_eager_expr_region))) =
            self.body_with_hir_expr_region(db)
        else {
            return;
        };
        self.hir_decl(db).transpile_to_rust(builder);
        builder.eager_body(body_hir_eager_expr_region, body)
    }
}

impl TranspileToRustWith for MajorFunctionRitchieHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let HirExprRegion::Eager(hir_eager_expr_region) = self.hir_expr_region(db) else {
            return;
        };
        builder.with_hir_eager_expr_region(hir_eager_expr_region, |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            self.path(db).ident(db).transpile_to_rust(builder);
            self.template_parameters(db).transpile_to_rust(builder);
            self.parenate_parameters(db)
                .eager()
                .transpile_to_rust(builder);
            builder.return_ty(self.return_ty(db))
        })
    }
}

impl TranspileToRustWith for MajorValHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl(db);
        let hir_expr_body_and_region = self.hir_expr_body_and_region(db).unwrap();
        let is_lazy = match hir_expr_body_and_region {
            (HirExprIdx::Eager(_body), HirExprRegion::Eager(_hir_eager_expr_region)) => false,
            _ => true,
        };
        builder.val_item_attr(
            hir_decl.path(db).into(),
            is_lazy,
            hir_decl.return_ty(db).always_copyable(db),
        );
        hir_decl.transpile_to_rust(builder);
        match self.hir_expr_body_and_region(db).unwrap() {
            (HirExprIdx::Eager(body), HirExprRegion::Eager(hir_eager_expr_region)) => {
                builder.eager_body(hir_eager_expr_region, body)
            }
            _ => builder.omitted_curly_block(),
        }
    }
}

impl TranspileToRustWith for MajorValHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let hir_eager_expr_region = self.hir_eager_expr_region(builder.db());
        builder.with_hir_eager_expr_region(hir_eager_expr_region, |builder| {
            builder.keyword(RustKeyword::Pub);
            builder.keyword(RustKeyword::Fn);
            let db = builder.db();
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |_| ());
            builder.return_ty(self.return_ty(db))
        })
    }
}
