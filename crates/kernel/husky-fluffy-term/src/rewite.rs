use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
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

impl FluffyTerm {
    pub fn substitute_variable(
        self,
        engine: &mut impl FluffyTermEngine,
        src: HoleSource,
        variable: FluffyTerm,
        substitute: FluffyTerm,
    ) -> Self {
        self.rewrite_inner(
            engine.db(),
            engine.fluffy_terms_mut(),
            src,
            &[ImplicitParameterSubstitution::new(variable, substitute)],
        )
    }

    pub(crate) fn rewrite_inner(
        self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        src: HoleSource,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> Self {
        if substitution_rules.len() == 0 {
            return self;
        }
        self.rewrite_aux(db, terms, src, substitution_rules)
    }

    fn rewrite_aux(
        self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        src: HoleSource,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> FluffyTerm {
        assert!(substitution_rules.len() > 0);
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path,
                ty_arguments: arguments,
                ..
            } => {
                let arguments: SmallVec<[FluffyTerm; 2]> = arguments.to_smallvec();
                let arguments = arguments
                    .into_iter()
                    .map(|argument| argument.rewrite_inner(db, terms, src, substitution_rules))
                    .collect();
                FluffyTerm::new_ty_ontology(db, terms, path, refined_ty_path, arguments)
            }
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                let parameter_variable =
                    parameter_variable.map(|v| v.rewrite_inner(db, terms, src, substitution_rules));
                let parameter_ty = parameter_ty.rewrite_inner(db, terms, src, substitution_rules);
                let return_ty = return_ty.rewrite_inner(db, terms, src, substitution_rules);
                FluffyTerm::new_curry(
                    db,
                    terms,
                    curry_kind,
                    variance,
                    parameter_variable,
                    parameter_ty,
                    return_ty,
                )
            }
            FluffyTermData::Hole(_, _) => self,
            FluffyTermData::Category(_) => self,
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => {
                let mut parameter_contracted_tys = parameter_contracted_tys.to_vec();
                for parameter_contracted_ty in &mut parameter_contracted_tys {
                    *parameter_contracted_ty.ty_mut() = parameter_contracted_ty.ty().rewrite_inner(
                        db,
                        terms,
                        src,
                        substitution_rules,
                    );
                }
                let return_ty = return_ty.rewrite_inner(db, terms, src, substitution_rules);
                FluffyTerm::new_richie(db, terms, ritchie_kind, parameter_contracted_tys, return_ty)
            }
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Variable { ty } => substitution_rules
                .iter()
                .copied()
                .find_map(|rule| (rule.variable == self).then_some(rule.substitute))
                .unwrap_or(self),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
