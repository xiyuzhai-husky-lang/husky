use super::*;
use husky_vfs::Toolchain;

#[inline(always)]
pub fn raw_term_raw_ty(
    db: &dyn DeclarativeTypeDb,
    _disambiguation: TypePathDisambiguation,
    raw_term: DeclarativeTerm,
    _toolchain: Toolchain,
    declarative_term_menu: DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    match raw_term {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(_) => todo!(),
        DeclarativeTerm::Hole(_) => todo!(),
        DeclarativeTerm::EntityPath(path) => raw_term_entity_path_raw_ty(db, path),
        DeclarativeTerm::Category(cat) => cat.ty().map(Into::into).map_err(|_e| todo!()),
        DeclarativeTerm::Universe(_) => todo!(),
        DeclarativeTerm::Curry(_) => todo!(),
        DeclarativeTerm::Ritchie(raw_term) => Ok(match raw_term.ritchie_kind(db) {
            TermRitchieKind::FnType => declarative_term_menu.ty0().into(),
            TermRitchieKind::FnTrait | TermRitchieKind::FnMutTrait => {
                declarative_term_menu.trai_ty()
            }
        }),
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::ExplicitApplication(raw_term) => application_raw_term_raw_ty(db, raw_term),
        DeclarativeTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        DeclarativeTerm::Subentity(_) => todo!(),
        DeclarativeTerm::AsTraitSubentity(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
        DeclarativeTerm::List(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub(crate) fn application_raw_term_raw_ty(
    _db: &dyn DeclarativeTypeDb,
    _raw_term: DeclarativeTermExplicitApplication,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Err(OriginalDeclarativeTypeError::Todo.into())
}
