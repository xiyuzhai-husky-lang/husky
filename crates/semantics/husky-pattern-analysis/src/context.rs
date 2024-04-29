use self::error::FlyPatternAnalysisError;
use crate::*;
use husky_entity_path::TypeVariantIndex;
use husky_fly_term::FlyTerm;
use husky_lifetime_utils::capture::Captures;

#[derive(Debug)]
pub struct PatternAnalysisContext {}

impl IsPatternAnalyisContext for PatternAnalysisContext {
    type Type = FlyTerm;

    type Error = FlyPatternAnalysisError;

    type VariantIdx = TypeVariantIndex;

    type StringLiteral = ();

    type MatchArmData = ();

    type PatternDataExtra = ();

    fn is_exhaustive_patterns_feature_on(&self) -> bool {
        todo!()
    }

    fn is_min_exhaustive_patterns_feature_on(&self) -> bool {
        todo!()
    }

    fn constructor_arity(
        &self,
        constructor: &rustc_pattern_analysis::constructor::Constructor<Self>,
        ty: &Self::Type,
    ) -> usize {
        todo!()
    }

    fn constructor_field_tys<'a>(
        &'a self,
        constructor: &'a rustc_pattern_analysis::constructor::Constructor<Self>,
        ty: &'a Self::Type,
    ) -> impl Iterator<Item = (Self::Type, rustc_pattern_analysis::PrivateUninhabitedField)>
           + ExactSizeIterator
           + Captures<'a> {
        [].into_iter().map(|_: i32| todo!())
    }

    fn constructors_for_ty(
        &self,
        ty: &Self::Type,
    ) -> Result<rustc_pattern_analysis::constructor::ConstructorSet<Self>, Self::Error> {
        todo!()
    }

    fn write_variant_name(
        f: &mut std::fmt::Formatter<'_>,
        constructor: &rustc_pattern_analysis::constructor::Constructor<Self>,
        ty: &Self::Type,
    ) -> std::fmt::Result {
        todo!()
    }

    fn bug(&self, fmt: std::fmt::Arguments<'_>) -> Self::Error {
        todo!()
    }

    fn complexity_exceeded(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
