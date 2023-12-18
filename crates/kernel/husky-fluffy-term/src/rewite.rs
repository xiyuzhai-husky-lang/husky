use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ImplicitParameterSubstitution {
    rune: FluffyTermRune,
    substitute: FluffyTerm,
}

impl ImplicitParameterSubstitution {
    pub(crate) fn new(rune: FluffyTermRune, substitute: impl Into<FluffyTerm>) -> Self {
        Self {
            rune,
            substitute: substitute.into(),
        }
    }
}

impl FluffyTerm {
    pub fn substitute_rune(
        self,
        engine: &mut impl FluffyTermEngine,
        src: HoleSource,
        rune: FluffyTermRune,
        substitute: FluffyTerm,
    ) -> Self {
        self.rewrite_inner(
            engine.db(),
            engine.fluffy_terms_mut(),
            src,
            &[ImplicitParameterSubstitution::new(rune, substitute)],
        )
    }

    pub(crate) fn rewrite_inner(
        self,
        db: &::salsa::Db,
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
        db: &::salsa::Db,
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
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                p!(substitution_rules, parameter_rune);
                let parameter_rune =
                    parameter_rune.map(|v| v.rewrite_inner(db, terms, src, substitution_rules));
                let parameter_ty = parameter_ty.rewrite_inner(db, terms, src, substitution_rules);
                let return_ty = return_ty.rewrite_inner(db, terms, src, substitution_rules);
                FluffyTerm::new_curry(
                    db,
                    terms,
                    curry_kind,
                    variance,
                    parameter_rune,
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
            // todo: this is wrong
            FluffyTermData::Rune { .. } => substitution_rules
                .iter()
                .copied()
                .find_map(|rule| (*rule.rune == self).then_some(rule.substitute))
                .unwrap_or(self),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
