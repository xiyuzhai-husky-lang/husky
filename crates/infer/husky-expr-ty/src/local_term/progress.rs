use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermResolveProgress {
    Unresolved,
    Resolved {
        implicit_conversion: LocalTermImplicitConversion,
        term: ReducedTerm,
    },
    Err(ExprTypeError),
}

impl LocalTermResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self) -> Self {
        match self {
            LocalTermResolveProgress::Unresolved => LocalTermResolveProgress::Unresolved,
            LocalTermResolveProgress::Resolved {
                implicit_conversion,
                term,
            } => LocalTermResolveProgress::Resolved {
                implicit_conversion: *implicit_conversion,
                term: *term,
            },
            LocalTermResolveProgress::Err(_) => {
                LocalTermResolveProgress::Err(DerivedExprTypeError::LocalTermResolveError.into())
            }
        }
    }

    pub(crate) fn term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermResolveProgress::Unresolved => None,
            LocalTermResolveProgress::Resolved { term, .. } => Some(*term),
            LocalTermResolveProgress::Err(_) => None,
        }
    }
}
