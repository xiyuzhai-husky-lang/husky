use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRune(FluffyTerm);

impl std::ops::Deref for FluffyTermRune {
    type Target = FluffyTerm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FluffyTermRune {
    pub(crate) fn rewrite_inner(
        self,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
        src: HoleSource,
        substitution_rules: &[ImplicitParameterSubstitution],
    ) -> Self {
        let slf = (*self).rewrite_inner(db, terms, src, substitution_rules);
        match slf.base_ty_data_inner(db, terms) {
            FluffyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyBaseTypeData::Hole(_, _) => todo!(),
            FluffyBaseTypeData::Category(_) => todo!(),
            FluffyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyBaseTypeData::Symbol { symbol } => todo!(),
            FluffyBaseTypeData::Rune { rune } => (),
        }
        Self(slf)
    }
}

impl From<EtherealTermRune> for FluffyTermRune {
    fn from(value: EtherealTermRune) -> Self {
        FluffyTermRune(value.into())
    }
}
