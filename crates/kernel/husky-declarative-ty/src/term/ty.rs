use super::*;
use husky_vfs::Toolchain;

#[inline(always)]
pub fn declarative_term_declarative_ty(
    db: &::salsa::Db,
    _disambiguation: TypePathDisambiguation,
    declarative_term: DeclarativeTerm,
    _toolchain: Toolchain,
    declarative_term_menu: DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    match declarative_term {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(_) => todo!(),
        DeclarativeTerm::Rune(_) => todo!(),
        DeclarativeTerm::EntityPath(path) => declarative_term_item_path_declarative_ty(db, path),
        DeclarativeTerm::Category(cat) => cat.ty().map(Into::into).map_err(|_e| todo!()),
        DeclarativeTerm::Universe(_) => todo!(),
        DeclarativeTerm::Curry(_) => todo!(),
        DeclarativeTerm::Ritchie(declarative_term) => Ok(match declarative_term.ritchie_kind(db) {
            RitchieKind::Type(_) => declarative_term_menu.ty0().into(),
            RitchieKind::Trait(_) => declarative_term_menu.trai_ty(),
        }),
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::ExplicitApplication(declarative_term) => {
            application_declarative_term_declarative_ty(db, declarative_term)
        }
        DeclarativeTerm::ExplicitApplicationOrRitchieCall(_declarative_ty) => todo!(),
        DeclarativeTerm::Subitem(_) => todo!(),
        DeclarativeTerm::AsTraitSubitem(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
        DeclarativeTerm::List(_) => todo!(),
        DeclarativeTerm::Wrapper(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub(crate) fn application_declarative_term_declarative_ty(
    _db: &::salsa::Db,
    _declarative_term: DeclarativeTermExplicitApplication,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Err(OriginalDeclarativeTypeError::Todo.into())
}
