use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ImplicitParameterSubstitution {
    rune: FluffyTermRune,
    substitute: FluffyTerm,
}

pub type ImplicitParameterSubstitutions = SmallVec<[ImplicitParameterSubstitution; 2]>;

impl ImplicitParameterSubstitution {
    fn new(rune: FluffyTermRune, substitute: impl Into<FluffyTerm>) -> Self {
        Self {
            rune,
            substitute: substitute.into(),
        }
    }

    /// this will collect implicit parameters and give rules that replace them with holes
    pub(crate) fn from_expectee(
        expectee: FluffyTerm,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
    ) -> (FluffyTerm, ImplicitParameterSubstitutions) {
        Self::from_expectee_aux(expectee, db, terms, idx, smallvec![])
    }

    fn from_expectee_aux(
        expectee: FluffyTerm,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        idx: FluffyTermExpectationIdx,
        mut template_parameter_substitutions: ImplicitParameterSubstitutions,
    ) -> (FluffyTerm, ImplicitParameterSubstitutions) {
        match expectee.base_ty_data_inner(db, terms) {
            FluffyBaseTypeData::Curry {
                curry_kind: CurryKind::Implicit,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                let parameter_rune = parameter_rune
                    .expect("curry type with implicit parameter should be dependent type");
                let implicit_symbol = terms.new_hole_from_parameter_rune(
                    db,
                    HoleSource::Expectation(idx),
                    parameter_rune,
                );
                template_parameter_substitutions.push(ImplicitParameterSubstitution::new(
                    parameter_rune,
                    implicit_symbol,
                ));
                let expectee = return_ty.rewrite_inner(
                    db,
                    terms,
                    HoleSource::Expectation(idx),
                    &template_parameter_substitutions,
                );
                Self::from_expectee_aux(expectee, db, terms, idx, template_parameter_substitutions)
            }
            _ => (expectee, template_parameter_substitutions),
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
