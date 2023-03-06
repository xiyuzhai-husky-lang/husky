use super::*;
use husky_vfs::Toolchain;

#[inline(always)]
pub fn term_ty(
    db: &dyn TypeDb,
    disambiguation: TypePathDisambiguation,
    reduced_term: Term,
    toolchain: Toolchain,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    match reduced_term {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::EntityPath(path) => term_entity_path_ty(db, path),
        Term::Category(cat) => cat.ty().map(Into::into).map_err(|e| todo!()),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(term) => Ok(match term.ritchie_kind(db) {
            TermRitchieKind::Fp => term_menu.ty0().into(),
            TermRitchieKind::Fn | TermRitchieKind::FnMut => term_menu.trai_ty(),
        }),
        Term::Abstraction(_) => todo!(),
        Term::Application(term) => application_term_ty(db, term),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn application_term_ty(db: &dyn TypeDb, term: TermApplication) -> TypeResult<Term> {
    Err(OriginalTypeError::Todo.into())
}
