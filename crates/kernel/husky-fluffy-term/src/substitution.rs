use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ImplicitParameterSubstitution {
    symbol: FluffyTerm,
    substitute: FluffyTerm,
}

impl ImplicitParameterSubstitution {
    pub(crate) fn new(symbol: FluffyTerm, substitute: impl Into<FluffyTerm>) -> Self {
        Self {
            symbol,
            substitute: substitute.into(),
        }
    }
}

impl FluffyTerms {
    pub(crate) fn substitute_into_term(
        &mut self,
        db: &dyn FluffyTermDb,
        src: HoleSource,
        term: FluffyTerm,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> FluffyTerm {
        if substitution_rules.len() == 0 {
            return term;
        }
        self.substitute_into_term_aux(db, src, term, substitution_rules)
    }

    fn substitute_into_term_aux(
        &mut self,
        db: &dyn FluffyTermDb,
        src: HoleSource,
        term: FluffyTerm,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> FluffyTerm {
        assert!(substitution_rules.len() > 0);
        match term.data_inner(db, self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                argument_tys: arguments,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                argument_tys,
            } => todo!(),
        }
    }
}
