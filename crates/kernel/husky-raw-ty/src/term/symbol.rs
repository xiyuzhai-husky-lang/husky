use vec_like::VecSet;

use super::*;

#[salsa::tracked(db = RawTypeDb, jar = RawTypeJar)]
pub struct RawTermSymbols {
    #[return_ref]
    data: VecSet<RawTermSymbol>,
}

impl RawTermSymbols {
    pub(crate) fn contains(self, db: &dyn RawTypeDb, symbol: RawTermSymbol) -> bool {
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
}

pub(crate) fn calc_raw_term_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTerm,
) -> Option<RawTermSymbols> {
    match raw_term {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(symbol) => Some(RawTermSymbols::new(db, VecSet::new_one_elem_set(symbol))),
        RawTerm::EntityPath(path) => match path {
            RawTermEntityPath::Form(_) => todo!(),
            RawTermEntityPath::Trait(_)
            | RawTermEntityPath::TypeOntology(_)
            | RawTermEntityPath::TypeConstructor(_) => None,
        },
        RawTerm::Category(_) => None,
        RawTerm::Universe(_) => None,
        RawTerm::Curry(raw_term) => raw_term_curry_symbols(db, raw_term),
        RawTerm::Ritchie(raw_term) => raw_term_ritchie_symbols(db, raw_term),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::Application(raw_term) => raw_term_application_symbols(db, raw_term),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_curry_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermCurry,
) -> Option<RawTermSymbols> {
    todo!()
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_ritchie_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermRitchie,
) -> Option<RawTermSymbols> {
    let mut symbols: Option<RawTermSymbols> = None;
    for parameter_raw_ty in raw_term.parameter_tys(db) {
        symbols = RawTermSymbols::merge(symbols, calc_raw_term_symbols(db, parameter_raw_ty.ty()))
    }
    RawTermSymbols::merge(symbols, calc_raw_term_symbols(db, raw_term.return_ty(db)))
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_application_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermApplication,
) -> Option<RawTermSymbols> {
    RawTermSymbols::merge(
        calc_raw_term_symbols(db, raw_term.function(db)),
        calc_raw_term_symbols(db, raw_term.argument(db)),
    )
}
