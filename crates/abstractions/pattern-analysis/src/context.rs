use crate::constructor::IntRange;

use crate::pattern::DeconstructedPattern;

use crate::constructor::ConstructorSet;

use super::Captures;

use super::PrivateUninhabitedField;

use crate::constructor::Constructor;

use rustc_index::Idx;

use std::fmt;

/// Context that provides type information about constructors.
///
/// Most of the crate is parameterized on a type that implements this trait.
pub trait IsPatternAnalyisContext: Sized + fmt::Debug {
    /// The type of a pattern.
    type Type: Clone + fmt::Debug;
    /// Errors that can abort analysis.
    type Error: fmt::Debug;
    /// The index of an enum variant.
    type VariantIdx: Clone + Idx + fmt::Debug;
    /// A string literal
    type StringLiteral: Clone + PartialEq + fmt::Debug;
    /// Extra data to store in a match arm.
    type MatchArmData: Copy + Clone + fmt::Debug;
    /// Extra data to store in a pattern.
    type PatternDataExtra: Clone;

    fn is_exhaustive_patterns_feature_on(&self) -> bool;
    fn is_min_exhaustive_patterns_feature_on(&self) -> bool;

    /// The number of fields for this constructor.
    fn constructor_arity(&self, constructor: &Constructor<Self>, ty: &Self::Type) -> usize;

    /// The types of the fields for this constructor. The result must contain `constructor_arity()` fields.
    fn constructor_field_tys<'a>(
        &'a self,
        constructor: &'a Constructor<Self>,
        ty: &'a Self::Type,
    ) -> impl Iterator<Item = (Self::Type, PrivateUninhabitedField)> + ExactSizeIterator + Captures<'a>;

    /// The set of all the constructors for `ty`.
    ///
    /// This must follow the invariants of `ConstructorSet`
    fn constructors_for_ty(&self, ty: &Self::Type) -> Result<ConstructorSet<Self>, Self::Error>;

    /// Write the name of the variant represented by `pat`. Used for the best-effort `Debug` impl of
    /// `DeconstructedPattern`. Only invoqued when `pat.constructor()` is `Struct | Variant(_) | UnionField`.
    fn write_variant_name(
        f: &mut fmt::Formatter<'_>,
        constructor: &crate::constructor::Constructor<Self>,
        ty: &Self::Type,
    ) -> fmt::Result;

    /// Raise a bug.
    fn bug(&self, fmt: fmt::Arguments<'_>) -> Self::Error;

    /// Lint that the range `pat` overlapped with all the ranges in `overlaps_with`, where the range
    /// they overlapped over is `overlaps_on`. We only detect singleton overlaps.
    /// The default implementation does nothing.
    fn lint_overlapping_range_endpoints(
        &self,
        _pat: &DeconstructedPattern<Self>,
        _overlaps_on: IntRange,
        _overlaps_with: &[&DeconstructedPattern<Self>],
    ) {
    }

    /// The maximum pattern complexity limit was reached.
    fn complexity_exceeded(&self) -> Result<(), Self::Error>;

    /// Lint that there is a gap `gap` between `pat` and all of `gapped_with` such that the gap is
    /// not matched by another range. If `gapped_with` is empty, then `gap` is `T::MAX`. We only
    /// detect singleton gaps.
    /// The default implementation does nothing.
    fn lint_non_contiguous_range_endpoints(
        &self,
        _pat: &DeconstructedPattern<Self>,
        _gap: IntRange,
        _gapped_with: &[&DeconstructedPattern<Self>],
    ) {
    }
}
