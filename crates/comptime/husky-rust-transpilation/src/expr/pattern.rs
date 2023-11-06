use super::*;

impl TranspileToRust for HirEagerPatternExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_pattern_expr_arena()) {
            HirEagerPatternExpr::Literal(_) => todo!(),
            HirEagerPatternExpr::Ident { ident } => ident.transpile_to_rust(builder),
            HirEagerPatternExpr::Unit(_) => todo!(),
            HirEagerPatternExpr::Tuple { path, fields } => todo!(),
            HirEagerPatternExpr::Props { path, fields } => todo!(),
            HirEagerPatternExpr::OneOf { options } => todo!(),
            HirEagerPatternExpr::Binding { ident, src } => todo!(),
            HirEagerPatternExpr::Range { start, end } => todo!(),
        }
    }
}
