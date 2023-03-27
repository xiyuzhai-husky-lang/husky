use vec_like::VecSet;

use super::*;

#[salsa::tracked(db = RawTypeDb, jar = RawTypeJar)]
pub struct RawTermOriginalSymbols {
    #[return_ref]
    data: VecSet<RawTermOriginalSymbol>,
}

impl RawTermOriginalSymbols {
    pub(crate) fn contains(self, db: &dyn RawTypeDb, symbol: RawTermOriginalSymbol) -> bool {
        self.data(db).has(symbol)
    }

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

    fn remove(
        symbols: impl Into<Option<Self>>,
        _symbol: impl Into<Option<RawTermOriginalSymbol>>,
    ) -> Option<Self> {
        let _symbols = symbols.into()?;
        todo!()
    }
}
impl<'a> dyn RawTypeDb + 'a {
    pub(crate) fn raw_term_contains_symbol(
        &self,
        raw_term: RawTerm,
        symbol: RawTermOriginalSymbol,
    ) -> bool {
        calc_raw_term_symbols(self, raw_term)
            .map(|raw_term_symbols| raw_term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }
}

fn calc_raw_term_symbols(db: &dyn RawTypeDb, raw_term: RawTerm) -> Option<RawTermOriginalSymbols> {
    match raw_term {
        RawTerm::Literal(_) => todo!(),
        RawTerm::ConcreteSymbol(symbol) => Some(RawTermOriginalSymbols::new(
            db,
            VecSet::new_one_elem_set(symbol),
        )),
        RawTerm::AbstractSymbol(symbol) => None,
        RawTerm::EntityPath(path) => match path {
            RawTermEntityPath::Form(_) => todo!(),
            RawTermEntityPath::Trait(_) | RawTermEntityPath::Type(_) => None,
        },
        RawTerm::Category(_) => None,
        RawTerm::Universe(_) => None,
        RawTerm::Curry(raw_term) => raw_term_curry_symbols(db, raw_term),
        RawTerm::Ritchie(raw_term) => raw_term_ritchie_symbols(db, raw_term),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::ExplicitApplication(raw_term) => raw_term_application_symbols(db, raw_term),
        RawTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
        RawTerm::LeashOrBitNot(_) => todo!(),
        RawTerm::List(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_curry_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermCurry,
) -> Option<RawTermOriginalSymbols> {
    let parameter_ty_symbols = calc_raw_term_symbols(db, raw_term.parameter_ty(db));
    let return_ty_symbols = calc_raw_term_symbols(db, raw_term.return_ty(db));
    RawTermOriginalSymbols::merge(
        parameter_ty_symbols,
        RawTermOriginalSymbols::remove(return_ty_symbols, raw_term.parameter_symbol(db)),
    )
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_ritchie_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermRitchie,
) -> Option<RawTermOriginalSymbols> {
    let mut symbols: Option<RawTermOriginalSymbols> = None;
    for parameter_raw_ty in raw_term.parameter_tys(db) {
        symbols =
            RawTermOriginalSymbols::merge(symbols, calc_raw_term_symbols(db, parameter_raw_ty.ty()))
    }
    RawTermOriginalSymbols::merge(symbols, calc_raw_term_symbols(db, raw_term.return_ty(db)))
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_term_application_symbols(
    db: &dyn RawTypeDb,
    raw_term: RawTermExplicitApplication,
) -> Option<RawTermOriginalSymbols> {
    RawTermOriginalSymbols::merge(
        calc_raw_term_symbols(db, raw_term.function(db)),
        calc_raw_term_symbols(db, raw_term.argument(db)),
    )
}
