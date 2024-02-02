use super::*;
use vec_like::VecSet;

#[salsa::tracked(db = EthTermDb, jar = EthTermJar)]
pub struct EthTermSymbols {
    #[return_ref]
    pub(crate) data: VecSet<SymbolEthTerm>,
}

impl EthTermSymbols {
    pub(crate) fn contains(self, db: &::salsa::Db, symbol: SymbolEthTerm) -> bool {
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
        _symbol: impl Into<Option<SymbolEthTerm>>,
    ) -> Option<Self> {
        let _symbols = symbols.into()?;
        todo!()
    }
}

impl EthTerm {
    pub(crate) fn symbols(self, db: &::salsa::Db) -> Option<EthTermSymbols> {
        match self {
            EthTerm::Literal(_)
            | EthTerm::Rune(_)
            | EthTerm::EntityPath(_)
            | EthTerm::Category(_) => None,
            EthTerm::Universe(_) => None, // ad hoc
            EthTerm::Symbol(symbol) => {
                Some(EthTermSymbols::new(db, VecSet::new_one_elem_set(symbol)))
            }
            EthTerm::Curry(term) => term_curry_symbols(db, term),
            EthTerm::Ritchie(term) => term_ritchie_symbols(db, term),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(term) => term_application_symbols(db, term),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn term_curry_symbols(db: &::salsa::Db, term: CurryEthTerm) -> Option<EthTermSymbols> {
    let parameter_ty_symbols = term.parameter_ty(db).symbols(db);
    let return_ty_symbols = term.return_ty(db).symbols(db);
    EthTermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn term_ritchie_symbols(
    db: &::salsa::Db,
    term: RitchieEthTerm,
) -> Option<EthTermSymbols> {
    let mut symbols: Option<EthTermSymbols> = None;
    for parameter_ty in term.parameter_contracted_tys(db) {
        symbols = EthTermSymbols::merge(symbols, parameter_ty.ty().symbols(db))
    }
    EthTermSymbols::merge(symbols, term.return_ty(db).symbols(db))
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn term_application_symbols(
    db: &::salsa::Db,
    term: ApplicationEthTerm,
) -> Option<EthTermSymbols> {
    EthTermSymbols::merge(term.function(db).symbols(db), term.argument(db).symbols(db))
}
