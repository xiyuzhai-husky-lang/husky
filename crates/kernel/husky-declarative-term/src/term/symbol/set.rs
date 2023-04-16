use super::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermSymbols {
    #[return_ref]
    symbols: VecSet<DeclarativeTermSymbol>,
}

impl DeclarativeTermSymbols {
    pub(crate) fn contains(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
    ) -> bool {
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
impl DeclarativeTerm {
    pub fn contains_symbol(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
    ) -> bool {
        calc_raw_term_symbols(db, self)
            .map(|raw_term_symbols| raw_term_symbols.contains(db, symbol))
            .unwrap_or_default()
    }
}

fn calc_raw_term_symbols(
    db: &dyn DeclarativeTermDb,
    raw_term: DeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    match raw_term {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(symbol) => Some(DeclarativeTermSymbols::new(
            db,
            VecSet::new_one_elem_set(symbol),
        )),
        DeclarativeTerm::Hole(symbol) => None,
        DeclarativeTerm::EntityPath(path) => match path {
            DeclarativeTermEntityPath::Form(_) => todo!(),
            DeclarativeTermEntityPath::Trait(_) | DeclarativeTermEntityPath::Type(_) => None,
        },
        DeclarativeTerm::Category(_) => None,
        DeclarativeTerm::Universe(_) => None,
        DeclarativeTerm::Curry(raw_term) => raw_term_curry_symbols(db, raw_term),
        DeclarativeTerm::Ritchie(raw_term) => raw_term_ritchie_symbols(db, raw_term),
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::ExplicitApplication(raw_term) => {
            raw_term_application_symbols(db, raw_term)
        }
        DeclarativeTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
        DeclarativeTerm::Subentity(_) => todo!(),
        DeclarativeTerm::AsTraitSubentity(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
        DeclarativeTerm::List(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_curry_symbols(
    db: &dyn DeclarativeTermDb,
    raw_term: DeclarativeTermCurry,
) -> Option<DeclarativeTermSymbols> {
    let parameter_ty_symbols = calc_raw_term_symbols(db, raw_term.parameter_ty(db));
    let return_ty_symbols = calc_raw_term_symbols(db, raw_term.return_ty(db));
    DeclarativeTermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_ritchie_symbols(
    db: &dyn DeclarativeTermDb,
    raw_term: DeclarativeTermRitchie,
) -> Option<DeclarativeTermSymbols> {
    let mut symbols: Option<DeclarativeTermSymbols> = None;
    for parameter_raw_ty in raw_term.parameter_tys(db) {
        symbols =
            DeclarativeTermSymbols::merge(symbols, calc_raw_term_symbols(db, parameter_raw_ty.ty()))
    }
    DeclarativeTermSymbols::merge(symbols, calc_raw_term_symbols(db, raw_term.return_ty(db)))
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_application_symbols(
    db: &dyn DeclarativeTermDb,
    raw_term: DeclarativeTermExplicitApplication,
) -> Option<DeclarativeTermSymbols> {
    DeclarativeTermSymbols::merge(
        calc_raw_term_symbols(db, raw_term.function(db)),
        calc_raw_term_symbols(db, raw_term.argument(db)),
    )
}
