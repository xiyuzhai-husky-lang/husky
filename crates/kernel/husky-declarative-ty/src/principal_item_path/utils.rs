use vec_like::{VecPairMap, VecSet};

use super::*;

pub(super) fn curry_from_generic_parameters(
    db: &dyn DeclarativeSignatureDb,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    generic_parameters: &[DeclarativeGenericParameter],
    term: impl Into<DeclarativeTerm>,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let mut term = term.into();
    debug_assert_eq!(variances.len(), generic_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), generic_parameters.iter()).rev()
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
