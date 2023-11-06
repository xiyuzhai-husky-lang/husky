use husky_term_prelude::SymbolModifier;

use super::*;

impl TranspileToRust for HirEagerPatternExprIdx {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self.data(builder.hir_eager_pattern_expr_arena()) {
            HirEagerPatternExpr::Literal(_) => todo!(),
            HirEagerPatternExpr::Ident {
                symbol_modifier,
                ident,
            } => {
                if let Some(symbol_modifier) = symbol_modifier {
                    match symbol_modifier {
                        SymbolModifier::None => (),
                        SymbolModifier::Mut => builder.keyword(RustKeyword::Mut),
                        SymbolModifier::RefMut => todo!(),
                        SymbolModifier::Const => todo!(),
                        SymbolModifier::Ambersand(_) => todo!(),
                        SymbolModifier::AmbersandMut(_) => todo!(),
                        SymbolModifier::Le => todo!(),
                        SymbolModifier::Tilde => todo!(),
                    }
                }
                ident.transpile_to_rust(builder)
            }
            HirEagerPatternExpr::Unit(_) => todo!(),
            HirEagerPatternExpr::Tuple { path, fields } => todo!(),
            HirEagerPatternExpr::Props { path, fields } => todo!(),
            HirEagerPatternExpr::OneOf { options } => todo!(),
            HirEagerPatternExpr::Binding { ident, src } => todo!(),
            HirEagerPatternExpr::Range { start, end } => todo!(),
        }
    }
}
