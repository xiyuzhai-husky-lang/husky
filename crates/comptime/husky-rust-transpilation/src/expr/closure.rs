use crate::{RustPunctuation, RustTranspilationBuilder, TranspileToRustWith};
use husky_hir_eager_expr::{
    closure_parameter::HirEagerClosureParameterPattern, HirEagerExprRegion,
};

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerClosureParameterPattern {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match *self {
            HirEagerClosureParameterPattern::Simple { pattern_idx, ty } => {
                pattern_idx.transpile_to_rust(builder);
                if let Some(ty) = ty {
                    builder.punctuation(RustPunctuation::Colon);
                    ty.transpile_to_rust(builder);
                }
            }
        }
    }
}
