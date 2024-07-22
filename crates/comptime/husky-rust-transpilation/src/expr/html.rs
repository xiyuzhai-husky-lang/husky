use super::*;
use husky_hir_eager_expr::HirEagerHtmlArgumentExpr;

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerHtmlArgumentExpr {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        builder.delimited(RustDelimiter::Par, |builder| {
            builder.str_literal(self.property_ident().data(db));
            builder.punctuation(RustPunctuation::CommaSpaced);
            (self.expr(), HirEagerExprRole::html_argument()).transpile_to_rust(builder)
        })
    }
}
