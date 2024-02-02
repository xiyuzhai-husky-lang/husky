use super::*;
use vec_like::VecSet;

/// unlike DecTermSymbols
/// Some(DecTermRunes { unaccounted_runes: Default::default() })
/// means different from None
///
/// the former implies that runes exists, but all accounted
#[salsa::tracked(db = DecTermDb, jar = DecTermJar)]
pub struct DecTermRunes {
    /// unaccounted means the rune is not declared within this term
    #[return_ref]
    pub unaccounted_runes: VecSet<RuneDecTerm>,
}

impl DecTermRunes {
    #[inline(always)]
    pub(crate) fn contains(self, db: &::salsa::Db, rune: RuneDecTerm) -> bool {
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
        _rune: impl Into<Option<RuneDecTerm>>,
    ) -> Option<Self> {
        let _runes = runes.into()?;
        todo!()
    }
}
impl DecTerm {
    pub fn contains_rune(self, db: &::salsa::Db, rune: RuneDecTerm) -> bool {
        self.runes(db)
            .map(|declarative_term_runes| declarative_term_runes.contains(db, rune))
            .unwrap_or_default()
    }

    pub(crate) fn runes(self, db: &::salsa::Db) -> Option<DecTermRunes> {
        match self {
            DecTerm::Literal(_) => todo!(),
            DecTerm::Rune(rune) => Some(DecTermRunes::new(db, VecSet::new_one_elem_set(rune))),
            DecTerm::Symbol(_symbol) => None,
            DecTerm::EntityPath(path) => match path {
                ItemPathDecTerm::Fugitive(_) => todo!(),
                ItemPathDecTerm::Trait(_) | ItemPathDecTerm::Type(_) => None,
                ItemPathDecTerm::TypeVariant(_) => todo!(),
            },
            DecTerm::Category(_) => None,
            DecTerm::Universe(_) => None,
            DecTerm::Curry(declarative_term) => {
                declarative_term_curry_placeholders(db, declarative_term)
            }
            DecTerm::Ritchie(declarative_term) => {
                declarative_term_ritchie_runes(db, declarative_term)
            }
            DecTerm::Abstraction(_) => todo!(),
            DecTerm::Application(declarative_term) => {
                declarative_term_application_runes(db, declarative_term)
            }
            DecTerm::ApplicationOrRitchieCall(_declarative_ty) => todo!(),
            DecTerm::AssociatedItem(_) => todo!(),
            DecTerm::TypeAsTraitItem(_) => todo!(),
            DecTerm::TraitConstraint(_) => todo!(),
            DecTerm::LeashOrBitNot(_) => todo!(),
            DecTerm::List(_) => todo!(),
            DecTerm::Wrapper(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn declarative_term_curry_placeholders(
    db: &::salsa::Db,
    term: CurryDecTerm,
) -> Option<DecTermRunes> {
    let parameter_ty_runes = term.parameter_ty(db).runes(db);
    let return_ty_runes = term.return_ty(db).runes(db);
    DecTermRunes::merge(
        parameter_ty_runes,
        DecTermRunes::remove(return_ty_runes, term.parameter_rune(db)),
    )
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn declarative_term_ritchie_runes(
    db: &::salsa::Db,
    term: RitchieDecTerm,
) -> Option<DecTermRunes> {
    let mut runes: Option<DecTermRunes> = None;
    for param in term.params(db) {
        runes = DecTermRunes::merge(runes, param.ty().runes(db))
    }
    DecTermRunes::merge(runes, term.return_ty(db).runes(db))
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn declarative_term_application_runes(
    db: &::salsa::Db,
    term: ApplicationDecTerm,
) -> Option<DecTermRunes> {
    DecTermRunes::merge(term.function(db).runes(db), term.argument(db).runes(db))
}
