use crate::constructor::{Constructor, SplitConstructorSet};
use crate::pattern::{DeconstructedPattern, PatOrWild};
use crate::{MatchArm, PatternContext};
use husky_lifetime_utils::capture::Captures;

/// A column of patterns in a match, where a column is the intuitive notion of "subpatterns that
/// inspect the same subvalue/place".
/// This is used to traverse patterns column-by-column for lints. Despite similarities with the
/// algorithm in [`crate::usefulness`], this does a different traversal. Notably this is linear in
/// the depth of patterns, whereas `compute_exhaustiveness_and_usefulness` is worst-case exponential
/// (exhaustiveness is NP-complete). The core difference is that we treat sub-columns separately.
///
/// This is not used in the usefulness algorithm; only in lints.
#[derive(Debug)]
pub struct PatternColumn<'p, Ctx: PatternContext> {
    /// This must not contain an or-pattern. `expand_and_push` takes care to expand them.
    patterns: Vec<&'p DeconstructedPattern<Ctx>>,
}

impl<'p, Ctx: PatternContext> PatternColumn<'p, Ctx> {
    pub fn new(arms: &[MatchArm<'p, Ctx>]) -> Self {
        let patterns = Vec::with_capacity(arms.len());
        let mut column = PatternColumn { patterns };
        for arm in arms {
            column.expand_and_push(PatOrWild::Pat(arm.pat));
        }
        column
    }
    /// Pushes a pattern onto the column, expanding any or-patterns into its subpatterns.
    /// Internal method, prefer [`PatternColumn::new`].
    fn expand_and_push(&mut self, pat: PatOrWild<'p, Ctx>) {
        // We flatten or-patterns and skip algorithm-generated wildcards.
        if pat.is_or_pat() {
            self.patterns.extend(
                pat.flatten_or_pat()
                    .into_iter()
                    .filter_map(|pat_or_wild| pat_or_wild.as_pat()),
            )
        } else if let Some(pat) = pat.as_pat() {
            self.patterns.push(pat)
        }
    }

    pub fn head_ty(&self) -> Option<&Ctx::PatternType> {
        self.patterns.first().map(|pat| pat.ty())
    }
    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'p DeconstructedPattern<Ctx>> + Captures<'a> {
        self.patterns.iter().copied()
    }

    /// Do constructor splitting on the constructors of the column.
    pub fn analyze_constructors(
        &self,
        cx: &Ctx,
        ty: &Ctx::PatternType,
    ) -> Result<SplitConstructorSet<Ctx>, Ctx::Error> {
        let column_constructors = self.patterns.iter().map(|p| p.constructor());
        let constructors_for_ty = cx.constructors_for_ty(ty)?;
        Ok(constructors_for_ty.split(column_constructors))
    }

    /// Does specialization: given a constructor, this takes the patterns from the column that match
    /// the constructor, and outputs their fields.
    /// This returns one column per field of the constructor. They usually all have the same length
    /// (the number of patterns in `self` that matched `constructor`), except that we expand or-patterns
    /// which may change the lengths.
    pub fn specialize(
        &self,
        cx: &Ctx,
        ty: &Ctx::PatternType,
        constructor: &Constructor<Ctx>,
    ) -> Vec<PatternColumn<'p, Ctx>> {
        let arity = constructor.arity(cx, ty);
        if arity == 0 {
            return Vec::new();
        }

        // We specialize the column by `constructor`. This gives us `arity`-many columns of patterns. These
        // columns may have different lengths in the presence of or-patterns (this is why we can't
        // reuse `Matrix`).
        let mut specialized_columns: Vec<_> = (0..arity)
            .map(|_| Self {
                patterns: Vec::new(),
            })
            .collect();
        let relevant_patterns = self.patterns.iter().filter(|pat| {
            constructor
                .is_covered_by(cx, pat.constructor())
                .unwrap_or(false)
        });
        for pat in relevant_patterns {
            let specialized = pat.specialize(constructor, arity);
            for (subpat, column) in specialized.into_iter().zip(&mut specialized_columns) {
                column.expand_and_push(subpat);
            }
        }
        specialized_columns
    }
}
