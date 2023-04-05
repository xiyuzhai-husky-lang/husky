use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermRitchie {
    ritchie_kind: TermRitchieKind,
    parameter_tys: Vec<LocalTermRitchieParameterLiasonedType>,
    return_ty: LocalTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermRitchieParameterLiasonedType {
    ty: LocalTerm,
}

impl LocalTermRitchieParameterLiasonedType {
    pub fn ty(self) -> LocalTerm {
        self.ty
    }
}

impl From<TermRitchieParameterLiasonedType> for LocalTermRitchieParameterLiasonedType {
    fn from(liasoned_ty: TermRitchieParameterLiasonedType) -> Self {
        Self {
            ty: liasoned_ty.ty().into(),
        }
    }
}
impl From<&TermRitchieParameterLiasonedType> for LocalTermRitchieParameterLiasonedType {
    fn from(liasoned_ty: &TermRitchieParameterLiasonedType) -> Self {
        Self {
            ty: liasoned_ty.ty().into(),
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
