use super::*;
use vec_like::VecSet;

#[salsa::tracked(db = TermDb, jar = TermJar)]
pub struct TermSymbols {
    #[return_ref]
    pub(crate) data: VecSet<TermSymbol>,
}

impl TermSymbols {
    pub(crate) fn contains(self, db: &dyn TermDb, symbol: TermSymbol) -> bool {
        self.data(db).has(symbol)
    }

    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(fst), Some(snd)) => todo!(),
        }
    }

    fn remove(
        symbols: impl Into<Option<Self>>,
        symbol: impl Into<Option<TermSymbol>>,
    ) -> Option<Self> {
        let symbols = symbols.into()?;
        todo!()
    }
}

impl Term {
    pub(crate) fn symbols(self, db: &dyn TermDb) -> Option<TermSymbols> {
        match self {
            Term::Literal(_) | Term::Hole(_) | Term::EntityPath(_) | Term::Category(_) => None,
            Term::Universe(_) => None, // ad hoc
            Term::Symbol(symbol) => Some(TermSymbols::new(db, VecSet::new_one_elem_set(symbol))),
            Term::Curry(term) => term_curry_symbols(db, term),
            Term::Ritchie(term) => term_ritchie_symbols(db, term),
            Term::Abstraction(_) => todo!(),
            Term::Application(term) => term_application_symbols(db, term),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
            Term::Place(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_curry_symbols(db: &dyn TermDb, term: TermCurry) -> Option<TermSymbols> {
    let parameter_ty_symbols = term.parameter_ty(db).symbols(db);
    let return_ty_symbols = term.return_ty(db).symbols(db);
    TermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_ritchie_symbols(db: &dyn TermDb, term: TermRitchie) -> Option<TermSymbols> {
    let mut symbols: Option<TermSymbols> = None;
    for parameter_ty in term.parameter_liasoned_tys(db) {
        symbols = TermSymbols::merge(symbols, parameter_ty.ty().symbols(db))
    }
    TermSymbols::merge(symbols, term.return_ty(db).symbols(db))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_symbols(
    db: &dyn TermDb,
    term: TermApplication,
) -> Option<TermSymbols> {
    TermSymbols::merge(term.function(db).symbols(db), term.argument(db).symbols(db))
}
