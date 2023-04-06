use husky_entity_path::TypePath;

use crate::utils::RawTermFamily;

use super::*;

impl RawTerm {
    /// the only way to create new variable
    /// this is not cached because
    /// - it's not called frequently
    /// - it's not computationally expensively
    pub fn turn_symbol_into_variable(
        self,
        db: &dyn RawTermDb,
        symbol: RawTermSymbol,
    ) -> (Self, Option<RawTermHole>) {
        let Some(idx) = self.new_variable_idx(db, symbol) else {
            return (self, None);
        };
        let variable = RawTermHole::new(db, symbol.ty(db), idx);
        (
            self.substitute_symbol_with_variable(db, symbol, variable),
            Some(variable),
        )
    }

    /// returns the variable idx if turning this symbol into variable
    /// returns None if symbol is not present
    #[inline(always)]
    fn new_variable_idx(self, db: &dyn RawTermDb, symbol: RawTermSymbol) -> Option<u8> {
        self.new_variable_idx_aux(db, symbol, symbol.ty_family(db))
    }

    /// with symbol_ty_family already fetched from db
    #[inline(always)]
    fn new_variable_idx_aux(
        self,
        db: &dyn RawTermDb,
        symbol: RawTermSymbol,
        symbol_ty_family: RawTermFamily,
    ) -> Option<u8> {
        self.contains_symbol(db, symbol)
            .then(|| self.new_variable_idx_if_symbol_is_present(db, symbol, symbol_ty_family))
    }

    // todo: needs thorough testing
    fn new_variable_idx_if_symbol_is_present(
        self,
        db: &dyn RawTermDb,
        symbol: RawTermSymbol,
        symbol_ty_family: RawTermFamily,
    ) -> u8 {
        let mut idx = 0;
        let mut t = |term: RawTerm| {
            if let Some(subidx) = term.new_variable_idx_aux(db, symbol, symbol_ty_family) {
                if subidx > idx {
                    idx = subidx
                }
            }
        };
        match self {
            RawTerm::Curry(_) => todo!(),
            RawTerm::Ritchie(term) => {
                for parameter_ty in term.parameter_tys(db) {
                    t(parameter_ty.ty())
                }
                t(term.return_ty(db));
            }
            RawTerm::Abstraction(term) => {
                let x = term.x(db);
                let m = term.m(db);
                t(m);
                if x.ty_family(db) == symbol_ty_family && m.contains_symbol(db, symbol) {
                    let x_idx = x.idx(db);
                    if x_idx > idx {
                        idx = x_idx
                    }
                }
            }
            RawTerm::ExplicitApplication(term) => {
                t(term.function(db));
                t(term.argument(db))
            }
            RawTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
            RawTerm::LeashOrBitNot(_) => todo!(),
            RawTerm::List(_) => todo!(),
            _ => (),
        }
        idx
    }

    // todo: needs thorough testing
    /// not cached on purpose
    fn substitute_symbol_with_variable(
        self,
        db: &dyn RawTermDb,
        symbol: RawTermSymbol,
        variable: RawTermHole,
    ) -> Self {
        if !self.contains_symbol(db, symbol) {
            return self;
        }
        match self {
            RawTerm::Symbol(term) if term == symbol => variable.into(),
            RawTerm::Universe(_) => self, // ad hoc
            RawTerm::Curry(term) => RawTermCurry::new(
                db,
                term.curry_kind(db),
                term.variance(db),
                term.parameter_variable(db),
                term.parameter_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
                term.return_ty(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            RawTerm::Ritchie(term) => RawTermRitchie::new(
                db,
                term.ritchie_kind(db),
                term.parameter_tys(db)
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
            RawTerm::Abstraction(term) => {
                let x = term.x(db);
                // should be equal by the choice of variable idx and the fact that m contains the symbol
                debug_assert_ne!(x, variable);
                RawTermAbstraction::new(
                    db,
                    x,
                    term.m(db)
                        .substitute_symbol_with_variable(db, symbol, variable),
                )
                .into()
            }
            RawTerm::ExplicitApplication(term) => RawTermExplicitApplication::new(
                db,
                term.function(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
                term.argument(db)
                    .substitute_symbol_with_variable(db, symbol, variable),
            )
            .into(),
            RawTerm::ExplicitApplicationOrRitchieCall(_) => todo!(),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
            RawTerm::LeashOrBitNot(_) => todo!(),
            RawTerm::List(_) => todo!(),
            _ => self,
        }
    }
}

fn variable_registry(
    db: &dyn RawTermDb,
    raw_term: RawTerm,
    symbol: RawTermSymbol,
) -> VariableRegistry {
    todo!()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct VariableRegistry {
    next: u8,
}

impl VariableRegistry {
    fn new(
        db: &dyn RawTermDb,
        variables: Option<RawTermPlaceholders>,
        ty_family: RawTermFamily,
    ) -> Self {
        let Some(variables) = variables else {
            return Default::default()
        };
        let mut next = 0;
        for variable in variables.unaccounted_variables(db).iter().copied() {
            // only need to disambiguous those with the same type family
            if variable.ty_family(db) == ty_family {
                // make sure that new variable won't conflict with this one
                let idx = variable.idx(db);
                if next <= idx {
                    next = idx + 1
                }
            }
        }
        Self { next }
    }

    pub(super) fn issue_variable_idx(self) -> u8 {
        self.next
    }

    fn merge(&mut self, other: Self) {
        todo!()
    }
}
