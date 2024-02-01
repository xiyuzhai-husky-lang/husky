use super::*;
use vec_like::VecSet;

/// unlike DeclarativeTermSymbols
/// Some(DeclarativeTermRunes { unaccounted_runes: Default::default() })
/// means different from None
///
/// the former implies that runes exists, but all accounted
#[salsa::tracked(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermRunes {
    /// unaccounted means the rune is not declared within this term
    #[return_ref]
    pub unaccounted_runes: VecSet<RuneDeclarativeTerm>,
}

impl DeclarativeTermRunes {
    #[inline(always)]
    pub(crate) fn contains(self, db: &::salsa::Db, rune: RuneDeclarativeTerm) -> bool {
        self.unaccounted_runes(db).has(rune)
    }

    #[inline(always)]
    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(_fst), Some(_snd)) => todo!(),
        }
    }

    #[inline(always)]
    fn remove(
        runes: impl Into<Option<Self>>,
        _rune: impl Into<Option<RuneDeclarativeTerm>>,
    ) -> Option<Self> {
        let _runes = runes.into()?;
        todo!()
    }
}
impl DeclarativeTerm {
    pub fn contains_rune(self, db: &::salsa::Db, rune: RuneDeclarativeTerm) -> bool {
        self.runes(db)
            .map(|declarative_term_runes| declarative_term_runes.contains(db, rune))
            .unwrap_or_default()
    }

    pub(crate) fn runes(self, db: &::salsa::Db) -> Option<DeclarativeTermRunes> {
        match self {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Rune(rune) => Some(DeclarativeTermRunes::new(
                db,
                VecSet::new_one_elem_set(rune),
            )),
            DeclarativeTerm::Symbol(_symbol) => None,
            DeclarativeTerm::EntityPath(path) => match path {
                ItemPathDeclarativeTerm::Fugitive(_) => todo!(),
                ItemPathDeclarativeTerm::Trait(_) | ItemPathDeclarativeTerm::Type(_) => None,
                ItemPathDeclarativeTerm::TypeVariant(_) => todo!(),
            },
            DeclarativeTerm::Category(_) => None,
            DeclarativeTerm::Universe(_) => None,
            DeclarativeTerm::Curry(declarative_term) => {
                declarative_term_curry_placeholders(db, declarative_term)
            }
            DeclarativeTerm::Ritchie(declarative_term) => {
                declarative_term_ritchie_runes(db, declarative_term)
            }
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::Application(declarative_term) => {
                declarative_term_application_runes(db, declarative_term)
            }
            DeclarativeTerm::ApplicationOrRitchieCall(_declarative_ty) => todo!(),
            DeclarativeTerm::AssociatedItem(_) => todo!(),
            DeclarativeTerm::TypeAsTraitItem(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_curry_placeholders(
    db: &::salsa::Db,
    term: CurryDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    let parameter_ty_runes = term.parameter_ty(db).runes(db);
    let return_ty_runes = term.return_ty(db).runes(db);
    DeclarativeTermRunes::merge(
        parameter_ty_runes,
        DeclarativeTermRunes::remove(return_ty_runes, term.parameter_rune(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_ritchie_runes(
    db: &::salsa::Db,
    term: RitchieDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    let mut runes: Option<DeclarativeTermRunes> = None;
    for param in term.params(db) {
        runes = DeclarativeTermRunes::merge(runes, param.ty().runes(db))
    }
    DeclarativeTermRunes::merge(runes, term.return_ty(db).runes(db))
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_application_runes(
    db: &::salsa::Db,
    term: ApplicationDeclarativeTerm,
) -> Option<DeclarativeTermRunes> {
    DeclarativeTermRunes::merge(term.function(db).runes(db), term.argument(db).runes(db))
}
