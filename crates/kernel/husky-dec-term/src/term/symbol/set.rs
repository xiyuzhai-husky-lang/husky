use super::*;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct DecTermSymbols {
    #[return_ref]
    symbols: VecSet<SymbolDecTerm>,
}

impl DecTermSymbols {
    pub(crate) fn contains(self, db: &::salsa::Db, symbol: SymbolDecTerm) -> bool {
        self.symbols(db).has(symbol)
    }

    fn merge(
        db: &::salsa::Db,
        fst: impl Into<Option<Self>>,
        snd: impl Into<Option<Self>>,
    ) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(fst), Some(snd)) => {
                let mut symbols = fst.symbols(db).clone();
                symbols.extend(snd.symbols(db).iter().copied());
                Some(DecTermSymbols::new(db, symbols))
            }
        }
    }
}
impl DecTerm {
    pub fn contains_symbol(self, db: &::salsa::Db, symbol: SymbolDecTerm) -> bool {
        calc_declarative_term_symbols(db, self)
            .map(|declarative_term_symbols| declarative_term_symbols.contains(db, symbol))
            .unwrap_or_default()
    }
}

fn calc_declarative_term_symbols(
    db: &::salsa::Db,
    declarative_term: DecTerm,
) -> Option<DecTermSymbols> {
    match declarative_term {
        DecTerm::Literal(_) => None,
        DecTerm::Symbol(symbol) => Some(DecTermSymbols::new(db, VecSet::new_one_elem_set(symbol))),
        DecTerm::Rune(_) => None,
        DecTerm::EntityPath(path) => match path {
            ItemPathDecTerm::Fugitive(_) => todo!(),
            ItemPathDecTerm::Trait(_) | ItemPathDecTerm::Type(_) => None,
            ItemPathDecTerm::TypeVariant(_) => todo!(),
        },
        DecTerm::Category(_) => None,
        DecTerm::Universe(_) => None,
        DecTerm::Curry(declarative_term) => declarative_term_curry_symbols(db, declarative_term),
        DecTerm::Ritchie(declarative_term) => {
            declarative_term_ritchie_symbols(db, declarative_term)
        }
        DecTerm::Abstraction(_) => todo!(),
        DecTerm::Application(declarative_term) => {
            application_declarative_term_symbols(db, declarative_term)
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

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn declarative_term_curry_symbols(
    db: &::salsa::Db,
    declarative_term: CurryDecTerm,
) -> Option<DecTermSymbols> {
    let parameter_ty_symbols = calc_declarative_term_symbols(db, declarative_term.parameter_ty(db));
    let return_ty_symbols = calc_declarative_term_symbols(db, declarative_term.return_ty(db));
    DecTermSymbols::merge(db, parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn declarative_term_ritchie_symbols(
    db: &::salsa::Db,
    declarative_term: RitchieDecTerm,
) -> Option<DecTermSymbols> {
    let mut symbols: Option<DecTermSymbols> = None;
    for param in declarative_term.params(db) {
        symbols = DecTermSymbols::merge(db, symbols, calc_declarative_term_symbols(db, param.ty()))
    }
    DecTermSymbols::merge(
        db,
        symbols,
        calc_declarative_term_symbols(db, declarative_term.return_ty(db)),
    )
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn application_declarative_term_symbols(
    db: &::salsa::Db,
    declarative_term: ApplicationDecTerm,
) -> Option<DecTermSymbols> {
    DecTermSymbols::merge(
        db,
        calc_declarative_term_symbols(db, declarative_term.function(db)),
        calc_declarative_term_symbols(db, declarative_term.argument(db)),
    )
}
