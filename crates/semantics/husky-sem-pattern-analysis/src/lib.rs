/// The entrypoint for this crate. Computes whether a match is exhaustive and which of its arms are
/// useful, and runs some lints.
#[cfg(feature = "rustc")]
pub fn analyze_match<'p, 'tcx>(
    tycx: &rustc::RustcPatCtxt<'p, 'tcx>,
    arms: &[rustc::MatchArm<'p, 'tcx>],
    scrut_ty: Ty<'tcx>,
    pattern_complexity_limit: Option<usize>,
) -> Result<rustc::UsefulnessReport<'p, 'tcx>, ErrorGuaranteed> {
    use lints::lint_nonexhaustive_missing_variants;
    use usefulness::{compute_match_usefulness, PlaceValidity};

    let scrut_ty = tycx.reveal_opaque_ty(scrut_ty);
    let scrut_validity = PlaceValidity::from_bool(tycx.known_valid_scrutinee);
    let report = compute_match_usefulness(
        tycx,
        arms,
        scrut_ty,
        scrut_validity,
        pattern_complexity_limit,
    )?;

    // Run the non_exhaustive_omitted_patterns lint. Only run on refutable patterns to avoid hitting
    // `if let`s. Only run if the match is exhaustive otherwise the error is redundant.
    if tycx.refutable && report.non_exhaustiveness_witnesses.is_empty() {
        let pat_column = PatternColumn::new(arms);
        lint_nonexhaustive_missing_variants(tycx, arms, &pattern_column, scrut_ty)?;
    }

    Ok(report)
}
