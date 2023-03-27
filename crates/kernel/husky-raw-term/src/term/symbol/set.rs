use super::*;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermSymbols {
    #[return_ref]
    symbols: VecSet<RawTermSymbol>,
}

impl RawTermSymbols {
    pub(crate) fn contains(self, db: &dyn RawTermDb, symbol: RawTermSymbol) -> bool {
        self.symbols(db).has(symbol)
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
}
impl RawTerm {
    pub fn contains_symbol(self, db: &dyn RawTermDb, symbol: RawTermSymbol) -> bool {
        calc_raw_term_symbols(db, self)
            .map(|raw_term_symbols| raw_term_symbols.contains(db, symbol))
            .unwrap_or_default()
    }
}

fn calc_raw_term_symbols(db: &dyn RawTermDb, raw_term: RawTerm) -> Option<RawTermSymbols> {
    match raw_term {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(symbol) => Some(RawTermSymbols::new(db, VecSet::new_one_elem_set(symbol))),
        RawTerm::Variable(symbol) => None,
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

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_curry_symbols(
    db: &dyn RawTermDb,
    raw_term: RawTermCurry,
) -> Option<RawTermSymbols> {
    let parameter_ty_symbols = calc_raw_term_symbols(db, raw_term.parameter_ty(db));
    let return_ty_symbols = calc_raw_term_symbols(db, raw_term.return_ty(db));
    RawTermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_ritchie_symbols(
    db: &dyn RawTermDb,
    raw_term: RawTermRitchie,
) -> Option<RawTermSymbols> {
    let mut symbols: Option<RawTermSymbols> = None;
    for parameter_raw_ty in raw_term.parameter_tys(db) {
        symbols = RawTermSymbols::merge(symbols, calc_raw_term_symbols(db, parameter_raw_ty.ty()))
    }
    RawTermSymbols::merge(symbols, calc_raw_term_symbols(db, raw_term.return_ty(db)))
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_application_symbols(
    db: &dyn RawTermDb,
    raw_term: RawTermExplicitApplication,
) -> Option<RawTermSymbols> {
    RawTermSymbols::merge(
        calc_raw_term_symbols(db, raw_term.function(db)),
        calc_raw_term_symbols(db, raw_term.argument(db)),
    )
}
