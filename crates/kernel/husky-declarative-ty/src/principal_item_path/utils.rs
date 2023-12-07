use super::*;
use husky_vfs::Toolchain;

pub(super) fn curry_from_template_parameters(
    db: &::salsa::Db,
    toolchain: Toolchain,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    template_parameters: &[DeclarativeTemplateParameter],
    term: impl Into<DeclarativeTerm>,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let mut term = term.into();
    debug_assert_eq!(variances.len(), template_parameters.len());
    for (variance, template_parameter) in
        std::iter::zip(variances.iter(), template_parameters.iter()).rev()
    {
        let symbol = template_parameter.symbol();
        term = DeclarativeTermCurry::new_dependent(
            db,
            toolchain,
            term_curry_kind,
            *variance,
            symbol,
            template_parameter.ty(db)?,
            term,
        )
        .into()
    }
    Ok(term)
}
