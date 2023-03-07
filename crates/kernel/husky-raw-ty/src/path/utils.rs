use super::*;

pub(super) fn curry_from_implicit_parameters(
    db: &dyn RawTypeDb,
    raw_term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    raw_term: impl Into<RawTerm>,
) -> RawTerm {
    let mut raw_term = raw_term.into();
    assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        let symbol = implicit_parameter.symbol();
        assert_eq!(symbol.ty(db), Ok(implicit_parameter.ty()));
        let symbol = db
            .raw_term_contains_symbol(raw_term, symbol)
            .then_some(symbol);
        raw_term = RawTermCurry::new(
            db,
            raw_term_curry_kind,
            *variance,
            symbol,
            implicit_parameter.ty(),
            raw_term,
        )
        .into()
    }
    raw_term
}
