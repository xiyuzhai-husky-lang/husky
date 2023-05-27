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
        calc_declarative_term_symbols(db, self)
            .map(|declarative_term_symbols| declarative_term_symbols.contains(db, symbol))
            .unwrap_or_default()
    }
}

fn calc_declarative_term_symbols(
    db: &dyn DeclarativeTermDb,
    declarative_term: DeclarativeTerm,
) -> Option<DeclarativeTermSymbols> {
    match declarative_term {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(symbol) => Some(DeclarativeTermSymbols::new(
            db,
            VecSet::new_one_elem_set(symbol),
        )),
        DeclarativeTerm::Variable(symbol) => None,
        DeclarativeTerm::EntityPath(path) => match path {
            DeclarativeTermEntityPath::Form(_) => todo!(),
            DeclarativeTermEntityPath::Trait(_) | DeclarativeTermEntityPath::Type(_) => None,
        },
        DeclarativeTerm::Category(_) => None,
        DeclarativeTerm::Universe(_) => None,
        DeclarativeTerm::Curry(declarative_term) => {
            declarative_term_curry_symbols(db, declarative_term)
        }
        DeclarativeTerm::Ritchie(declarative_term) => {
            declarative_term_ritchie_symbols(db, declarative_term)
        }
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::ExplicitApplication(declarative_term) => {
            declarative_term_application_symbols(db, declarative_term)
        }
        DeclarativeTerm::ExplicitApplicationOrRitchieCall(_declarative_ty) => todo!(),
        DeclarativeTerm::Subentity(_) => todo!(),
        DeclarativeTerm::AsTraitSubentity(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
        DeclarativeTerm::LeashOrBitNot(_) => todo!(),
        DeclarativeTerm::List(_) => todo!(),
        DeclarativeTerm::Wrapper(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_curry_symbols(
    db: &dyn DeclarativeTermDb,
    declarative_term: DeclarativeTermCurry,
) -> Option<DeclarativeTermSymbols> {
    let parameter_ty_symbols = calc_declarative_term_symbols(db, declarative_term.parameter_ty(db));
    let return_ty_symbols = calc_declarative_term_symbols(db, declarative_term.return_ty(db));
    DeclarativeTermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_ritchie_symbols(
    db: &dyn DeclarativeTermDb,
    declarative_term: DeclarativeTermRitchie,
) -> Option<DeclarativeTermSymbols> {
    let mut symbols: Option<DeclarativeTermSymbols> = None;
    for parameter_declarative_ty in declarative_term.parameter_tys(db) {
        symbols = DeclarativeTermSymbols::merge(
            symbols,
            calc_declarative_term_symbols(db, parameter_declarative_ty.ty()),
        )
    }
    DeclarativeTermSymbols::merge(
        symbols,
        calc_declarative_term_symbols(db, declarative_term.return_ty(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_application_symbols(
    db: &dyn DeclarativeTermDb,
    declarative_term: DeclarativeTermExplicitApplication,
) -> Option<DeclarativeTermSymbols> {
    DeclarativeTermSymbols::merge(
        calc_declarative_term_symbols(db, declarative_term.function(db)),
        calc_declarative_term_symbols(db, declarative_term.argument(db)),
    )
}
