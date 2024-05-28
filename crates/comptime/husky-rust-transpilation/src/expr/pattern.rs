use husky_print_utils::p;
use husky_term_prelude::VariableModifier;

use super::*;

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerPatternIdx {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        use salsa::DebugWithDb;
        match *self.entry(builder.hir_eager_pattern_expr_arena()).data() {
            HirEagerPatternData::Literal(lit) => lit.transpile_to_rust(builder),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => {
                if let Some(symbol_modifier) = symbol_modifier {
                    match symbol_modifier {
                        VariableModifier::Pure => (),
                        VariableModifier::Owned => todo!(),
                        VariableModifier::Mut => builder.keyword(RustKeyword::Mut),
                        VariableModifier::Ref => {
                            p!(ident.debug(db));
                            todo!()
                        }
                        VariableModifier::RefMut => {
                            builder.keyword(RustKeyword::Ref);
                            builder.keyword(RustKeyword::Mut)
                        }
                        VariableModifier::Compterm => todo!(),
                        VariableModifier::Ambersand(_) => todo!(),
                        VariableModifier::AmbersandMut(_) => todo!(),
                        VariableModifier::Le => todo!(),
                        VariableModifier::Tilde => todo!(),
                        VariableModifier::At => todo!(),
                    }
                }
                ident.transpile_to_rust(builder)
            }
            HirEagerPatternData::UnitPath(path) => path.transpile_to_rust(builder),
            HirEagerPatternData::Tuple { path: _, fields: _ } => todo!(),
            HirEagerPatternData::Props { path: _, fields: _ } => todo!(),
            HirEagerPatternData::OneOf { options } => {
                let mut start = true;
                for option in options {
                    if start {
                        start = false
                    } else {
                        builder.punctuation(RustPunctuation::PatternOr)
                    }
                    option.transpile_to_rust(builder)
                }
            }
            HirEagerPatternData::Binding { ident: _, src: _ } => todo!(),
            HirEagerPatternData::Range { start: _, end: _ } => todo!(),
            HirEagerPatternData::Some => builder.some_pattern(),
        }
    }
}
