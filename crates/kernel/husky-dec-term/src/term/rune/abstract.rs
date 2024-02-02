use crate::helpers::DecTermFamily;

use super::*;

impl DecTerm {
    // variable should only be created in curry or abstraction
    /// the only way to create new variable
    ///
    /// this is not cached because
    /// - it's not called frequently
    /// - it's not computationally
    ///
    /// `<v0: Type> -> A v0 s0` w.r.t `s0` gives `<v1: _> -> <v0: Type> -> A v0 v1`
    /// `<v0: Type> -> A s0` w.r.t `s0` gives `<v1: _> -> <v0: Type> -> A v1`
    ///
    /// In the above two cases, it's necessary to name `s0` as `v1` otherwise there would be ambiguity.
    ///
    /// `A s0 -> <v0: A s0> -> B v0` w.r.t `s0` gives `<v0: _> -> A v0 -> <v0: A v0> -> B v0`
    ///
    /// In the above case, it's okay to name `s0` as `v0` because the inner dependent type rune declaration overrides the previous one.
    ///
    /// Consider
    /// A s0 -> B (<v12: Type> -> ... -> C s0) (<v1: Type> -> ... -> C s0)
    ///
    /// todo: need thorough testing
    pub(in crate::term) fn create_rune(
        self,
        db: &::salsa::Db,
        symbol: SymbolDecTerm,
    ) -> (Self, Option<RuneDecTerm>) {
        let Some(idx) = self.new_variable_idx(db, symbol) else {
            return (self, None);
        };
        let variable = RuneDecTerm::new(symbol.ty(db), idx, db);
        (
            self.substitute_symbol_with_variable(db, symbol, variable),
            Some(variable),
        )
    }

    pub fn new_curry_variable() -> Self {
        todo!()
    }

    /// returns the variable idx if turning this symbol into variable
    /// returns None if symbol is not present
    #[inline(always)]
    fn new_variable_idx(self, db: &::salsa::Db, symbol: SymbolDecTerm) -> Option<u8> {
        self.new_variable_idx_with_ty_family(db, symbol, symbol.ty_family(db))
    }

    /// with symbol_ty_family already fetched from db
    #[inline(always)]
    fn new_variable_idx_with_ty_family(
        self,
        db: &::salsa::Db,
        symbol: SymbolDecTerm,
        symbol_ty_family: DecTermFamily,
    ) -> Option<u8> {
        self.contains_symbol(db, symbol).then(|| {
            self.new_variable_disambiguator_if_symbol_is_present(db, symbol, symbol_ty_family)
        })
    }

    // todo: needs thorough testing
    fn new_variable_disambiguator_if_symbol_is_present(
        self,
        db: &::salsa::Db,
        symbol: SymbolDecTerm,
        symbol_ty_family: DecTermFamily,
    ) -> u8 {
        let mut disambiguator = match self {
            DecTerm::Curry(curry)
                if let Some(rune) = curry.parameter_rune(db)
                    && curry.return_ty(db).contains_symbol(db, symbol) =>
            {
                rune.idx(db).disambiguator() + 1
            }
            _ => 0,
        };
        let mut t = |term: DecTerm| {
            if let Some(subidx) = term.new_variable_idx_with_ty_family(db, symbol, symbol_ty_family)
            {
                if subidx > disambiguator {
                    disambiguator = subidx
                }
            }
        };
        // scan
        match self {
            DecTerm::Curry(term) => {
                if let Some(v) = term.parameter_rune(db) {
                    if let Ok(ty) = v.ty(db) {
                        t(ty)
                    }
                }
                t(term.parameter_ty(db));
                t(term.return_ty(db));
            }
            DecTerm::Ritchie(term) => {
                for parameter_ty in term.params(db) {
                    t(parameter_ty.ty())
                }
                t(term.return_ty(db));
            }
            DecTerm::Abstraction(term) => {
                let x = term.x(db);
                let m = term.m(db);
                t(m);
                if x.ty_family(db) == symbol_ty_family && m.contains_symbol(db, symbol) {
                    let x_disambiguator = x.idx(db).disambiguator;
                    if x_disambiguator > disambiguator {
                        disambiguator = x_disambiguator
                    }
                }
            }
            DecTerm::Application(term) => {
                t(term.function(db));
                t(term.argument(db))
            }
            DecTerm::ApplicationOrRitchieCall(_) => todo!(),
            DecTerm::AssociatedItem(_) => todo!(),
            DecTerm::TypeAsTraitItem(_) => todo!(),
            DecTerm::TraitConstraint(_) => todo!(),
            DecTerm::LeashOrBitNot(_) => todo!(),
            DecTerm::List(_) => todo!(),
            DecTerm::Literal(_)
            | DecTerm::Symbol(_)
            | DecTerm::Rune(_)
            | DecTerm::EntityPath(_)
            | DecTerm::Category(_)
            | DecTerm::Universe(_) => (),
            DecTerm::Wrapper(_) => todo!(),
        }
        disambiguator
    }

    // todo: needs thorough testing
    /// not cached on purpose
    pub(in crate::term) fn substitute_symbol_with_variable(
        self,
        db: &::salsa::Db,
        symbol: SymbolDecTerm,
        variable: RuneDecTerm,
    ) -> Self {
        if !self.contains_symbol(db, symbol) {
            return self;
        }
        match self {
            DecTerm::Symbol(term) if term == symbol => variable.into(),
            DecTerm::Universe(_) => self, // ad hoc
            DecTerm::Curry(term) => term
                .substitute_symbol_with_variable(db, symbol, variable)
                .into(),
            DecTerm::Ritchie(term) => RitchieDecTerm::new(
                db,
                term.ritchie_kind(db),
                term.params(db)
                    .iter()
                    .map(|parameter_ty| {
                        parameter_ty.substitute_ty(|ty| {
                            ty.substitute_symbol_with_variable(db, symbol, variable)
                        })
                    })
                    .collect(),
                term.return_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            DecTerm::Abstraction(term) => {
                let x = term.x(db);
                // should be equal by the choice of variable idx and the fact that m contains the symbol
                debug_assert_ne!(x, variable);
                AbstractionDecTerm::new(
                    db,
                    x,
                    term.m(db)
                        .substitute_symbol_with_variable(db, symbol, variable),
                )
                .into()
            }
            DecTerm::Application(term) => ApplicationDecTerm::new(
                db,
                term.function(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
                term.argument(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            DecTerm::ApplicationOrRitchieCall(_) => todo!(),
            DecTerm::AssociatedItem(_) => todo!(),
            DecTerm::TypeAsTraitItem(_) => todo!(),
            DecTerm::TraitConstraint(_) => todo!(),
            DecTerm::LeashOrBitNot(_) => todo!(),
            DecTerm::List(_) => todo!(),
            _ => self,
        }
    }
}
