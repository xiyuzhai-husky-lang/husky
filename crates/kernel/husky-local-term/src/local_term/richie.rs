use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermRitchie {
    ritchie_kind: TermRitchieKind,
    parameter_tys: Vec<LocalTermRitchieParameterContractedType>,
    return_ty: LocalTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermRitchieParameterContractedType {
    ty: LocalTerm,
}

impl LocalTermRitchieParameterContractedType {
    pub fn ty(self) -> LocalTerm {
        self.ty
    }
}

impl From<TermRitchieParameterContractedType> for LocalTermRitchieParameterContractedType {
    fn from(contracted_ty: TermRitchieParameterContractedType) -> Self {
        Self {
            ty: contracted_ty.ty().into(),
        }
    }
}
impl From<&TermRitchieParameterContractedType> for LocalTermRitchieParameterContractedType {
    fn from(contracted_ty: &TermRitchieParameterContractedType) -> Self {
        Self {
            ty: contracted_ty.ty().into(),
        }
    }
}

impl LocalTermRitchie {
    pub(super) fn extract_implicit_symbol_dependencies(
        &self,
        unresolved_terms: &UnresolvedTerms,
        dependencies: &mut VecSet<UnresolvedTermIdx>,
    ) {
        let mut f = |term: LocalTerm| {
            unresolved_terms.extract_implicit_symbol_dependencies_aux(term, dependencies)
        };
        f(self.return_ty);
        self.parameter_tys
            .iter()
            .map(|parameter_ty| parameter_ty.ty)
            .for_each(f);
    }
}
