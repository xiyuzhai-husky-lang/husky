use super::*;
use husky_vfs::Toolchain;

#[inline(always)]
pub fn raw_term_raw_ty(
    db: &dyn RawTypeDb,
    _disambiguation: TypePathDisambiguation,
    raw_term: RawTerm,
    _toolchain: Toolchain,
    raw_term_menu: RawTermMenu,
) -> RawTypeResult<RawTerm> {
    match raw_term {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::EntityPath(path) => raw_term_entity_path_raw_ty(db, path),
        RawTerm::Category(cat) => cat.ty().map(Into::into).map_err(|_e| todo!()),
        RawTerm::Universe(_) => todo!(),
        RawTerm::Curry(_) => todo!(),
        RawTerm::Ritchie(raw_term) => Ok(match raw_term.ritchie_kind(db) {
            TermRitchieKind::Fp => raw_term_menu.ty0().into(),
            TermRitchieKind::Fn | TermRitchieKind::FnMut => raw_term_menu.trai_ty(),
        }),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::ExplicitApplication(raw_term) => application_raw_term_raw_ty(db, raw_term),
        RawTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
        RawTerm::LeashOrBitNot(_) => todo!(),
        RawTerm::List(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn application_raw_term_raw_ty(
    _db: &dyn RawTypeDb,
    _raw_term: RawTermExplicitApplication,
) -> RawTypeResult<RawTerm> {
    Err(OriginalRawTypeError::Todo.into())
}
