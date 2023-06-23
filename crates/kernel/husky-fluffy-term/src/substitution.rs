use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ImplicitParameterSubstitution {
    variable: FluffyTerm,
    substitute: FluffyTerm,
}

impl ImplicitParameterSubstitution {
    pub(crate) fn new(variable: FluffyTerm, substitute: impl Into<FluffyTerm>) -> Self {
        Self {
            variable,
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
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => {
                let mut arguments = arguments.to_smallvec();
                for argument in &mut arguments {
                    *argument = self.substitute_into_term(db, src, *argument, substitution_rules)
                }
                FluffyTerm::new_ty_ontology(db, self, path, refined_path, arguments)
            }
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => {
                let mut parameter_contracted_tys = parameter_contracted_tys.to_vec();
                for parameter_contracted_ty in &mut parameter_contracted_tys {
                    *parameter_contracted_ty.ty_mut() = self.substitute_into_term(
                        db,
                        src,
                        parameter_contracted_ty.ty(),
                        substitution_rules,
                    );
                }
                let return_ty = self.substitute_into_term(db, src, return_ty, substitution_rules);
                FluffyTerm::new_richie(db, self, ritchie_kind, parameter_contracted_tys, return_ty)
            }
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { ty } => todo!(),
            FluffyTermData::Variable { ty } => substitution_rules
                .iter()
                .copied()
                .find_map(|rule| (rule.variable == term).then_some(rule.substitute))
                .unwrap_or(term),
        }
    }
}
