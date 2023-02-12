use super::*;

/// for some lifetime `'a`, expect the term to be equal to `&mut 'a T`
/// for some T
///
/// T is referred to as the inner type
#[derive(Debug, Clone)]
pub(crate) struct ExpectRefMut {
    pub(crate) lifetime: LocalTerm,
}

impl ExpectLocalTerm for ExpectRefMut {
    type Result = ExpectRefMutResult;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

pub(crate) enum ExpectRefMutResult {
    ResolvedOk {
        term: LocalTerm,
        /// T
        inner_ty: LocalTerm,
    },
    ResolvedErr(LocalTermExpectationError),
}

impl From<ExpectRefMutResult> for LocalTermExpectationResult {
    fn from(value: ExpectRefMutResult) -> Self {
        todo!()
    }
}

impl From<ExpectRefMut> for LocalTermExpectationRuleVariant {
    fn from(value: ExpectRefMut) -> Self {
        LocalTermExpectationRuleVariant::RefMut { lifetime: todo!() }
    }
}
