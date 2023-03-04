use super::*;
use husky_vfs::Toolchain;

#[inline(always)]
pub fn raw_term_raw_ty(
    db: &dyn RawTypeDb,
    disambiguation: TypePathDisambiguation,
    reduced_raw_term: ReducedRawTerm,
    toolchain: Toolchain,
    reduced_raw_term_menu: ReducedRawTermMenu,
) -> RawTypeResult<ReducedRawTerm> {
    match reduced_raw_term.raw_term() {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::EntityPath(path) => raw_term_entity_path_raw_ty(db, path),
        RawTerm::Category(cat) => cat
            .ty()
            .map(Into::into)
            .map(|raw_term| calc_reduced_raw_term(db, raw_term))
            .map_err(|e| OriginalRawTypeError::RawTerm(e).into()),
        RawTerm::Universe(_) => todo!(),
        RawTerm::Curry(_) => todo!(),
        RawTerm::Ritchie(raw_term) => Ok(match raw_term.ritchie_kind(db) {
            RawTermRitchieKind::Fp => reduced_raw_term_menu.ty0(),
            RawTermRitchieKind::Fn | RawTermRitchieKind::FnMut => reduced_raw_term_menu.trai_ty(),
        }),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::Application(raw_term) => application_raw_term_raw_ty(db, raw_term),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn application_raw_term_raw_ty(
    db: &dyn RawTypeDb,
    raw_term: RawTermApplication,
) -> RawTypeResult<ReducedRawTerm> {
    Err(OriginalRawTypeError::Todo.into())
}
