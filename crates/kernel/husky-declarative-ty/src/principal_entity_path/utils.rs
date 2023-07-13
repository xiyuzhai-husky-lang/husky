use vec_like::{VecPairMap, VecSet};

use super::*;

pub(super) fn curry_from_implicit_parameters(
    db: &dyn DeclarativeSignatureDb,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[DeclarativeGenericParameter],
    term: impl Into<DeclarativeTerm>,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let mut term = term.into();
    debug_assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        let symbol = implicit_parameter.symbol();
        term = DeclarativeTermCurry::new_dependent(
            db,
            term_curry_kind,
            *variance,
            symbol,
            implicit_parameter.ty(db)?,
            term,
        )
        .into()
    }
    Ok(term)
}
